use dashmap::DashMap;
use parking_lot::RwLock;
use std::sync::Arc;
use std::collections::{HashMap, HashSet};

/// Intelligence engine that learns from responses and adapts discovery strategy
#[derive(Clone)]
pub struct IntelligenceEngine {
    /// Track response patterns for status codes
    status_patterns: Arc<DashMap<u16, StatusPattern>>,
    
    /// Track wildcard response signatures
    wildcard_signatures: Arc<RwLock<HashSet<String>>>,
    
    /// Track successful path patterns
    successful_patterns: Arc<DashMap<String, PatternScore>>,
    
    /// Track consistently failing prefixes/suffixes
    failing_patterns: Arc<DashMap<String, u32>>,
    
    /// Track technology fingerprints discovered
    tech_fingerprints: Arc<RwLock<HashSet<String>>>,
    
    /// Track naming conventions discovered
    naming_patterns: Arc<RwLock<Vec<String>>>,
    
    /// Dynamically excluded status codes
    excluded_codes: Arc<RwLock<HashSet<u16>>>,
    
    /// Request statistics for rate adaptation
    stats: Arc<RwLock<RequestStats>>,
}

#[derive(Debug, Clone)]
pub struct StatusPattern {
    pub count: u32,
    pub avg_size: f64,
    pub last_seen: std::time::Instant,
    pub is_informative: bool,
}

#[derive(Debug, Clone)]
pub struct PatternScore {
    pub hits: u32,
    pub misses: u32,
    pub confidence: f32,
    pub mutations: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RequestStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub avg_response_time: f64,
    pub current_rate: f64,
    pub discoveries: u64,
}

impl IntelligenceEngine {
    pub fn new() -> Self {
        Self {
            status_patterns: Arc::new(DashMap::new()),
            wildcard_signatures: Arc::new(RwLock::new(HashSet::new())),
            successful_patterns: Arc::new(DashMap::new()),
            failing_patterns: Arc::new(DashMap::new()),
            tech_fingerprints: Arc::new(RwLock::new(HashSet::new())),
            naming_patterns: Arc::new(RwLock::new(Vec::new())),
            // Exclude error and rate-limit codes by default - only show successful responses (200s)
            excluded_codes: Arc::new(RwLock::new(HashSet::from([
                404,  // Not Found
                403,  // Forbidden
                429,  // Too Many Requests
                500,  // Internal Server Error
                502,  // Bad Gateway
                504,  // Gateway Timeout
                405,  // Method Not Allowed
                501,  // Not Implemented
            ]))),
            stats: Arc::new(RwLock::new(RequestStats::default())),
        }
    }
    
    /// Learn from a response and update intelligence
    pub fn learn_from_response(
        &self,
        status: u16,
        size: usize,
        path: &str,
        body: &str,
        headers: &HashMap<String, String>,
    ) {
        // Update status pattern tracking
        self.status_patterns
            .entry(status)
            .and_modify(|pattern| {
                pattern.count += 1;
                pattern.avg_size = (pattern.avg_size * (pattern.count - 1) as f64 + size as f64)
                    / pattern.count as f64;
                pattern.last_seen = std::time::Instant::now();
                
                // Mark uninformative if seen too frequently with same size
                if pattern.count > 20 && (pattern.avg_size - size as f64).abs() < 10.0 {
                    pattern.is_informative = false;
                }
            })
            .or_insert(StatusPattern {
                count: 1,
                avg_size: size as f64,
                last_seen: std::time::Instant::now(),
                is_informative: true,
            });
        
        // Detect wildcard responses by content similarity
        let signature = self.generate_signature(size, body);
        if self.is_likely_wildcard(status, &signature) {
            self.wildcard_signatures.write().insert(signature);
            
            // Auto-exclude this status code if it's a wildcard
            if status != 200 && status != 301 && status != 302 {
                self.excluded_codes.write().insert(status);
            }
        }
        
        // Extract technology fingerprints from headers and body
        self.extract_tech_fingerprints(headers, body);
        
        // Learn from successful paths
        if self.is_successful_status(status) {
            self.learn_from_success(path);
        } else {
            self.learn_from_failure(path);
        }
        
        // Update statistics
        let mut stats = self.stats.write();
        stats.total_requests += 1;
        if self.is_successful_status(status) {
            stats.successful_requests += 1;
            stats.discoveries += 1;
        } else {
            stats.failed_requests += 1;
        }
    }
    
    /// Check if a response should be filtered out
    pub fn should_filter(&self, status: u16, size: usize, body: &str) -> bool {
        // Check excluded codes
        if self.excluded_codes.read().contains(&status) {
            return true;
        }
        
        // Check wildcard signature
        let signature = self.generate_signature(size, body);
        if self.wildcard_signatures.read().contains(&signature) {
            return true;
        }
        
        // Check if status pattern is uninformative
        if let Some(pattern) = self.status_patterns.get(&status) {
            if !pattern.is_informative && pattern.count > 50 {
                return true;
            }
        }
        
        false
    }
    
    /// Generate intelligent mutations based on successful patterns
    pub fn generate_mutations(&self, base_word: &str) -> Vec<String> {
        let mut mutations = Vec::new();
        
        // Extract patterns from successful findings
        let patterns = self.extract_successful_patterns();
        
        for pattern in patterns.iter().take(5) {
            // Generate variations based on learned patterns
            if pattern.contains('/') {
                let parts: Vec<&str> = pattern.split('/').collect();
                if let Some(last) = parts.last() {
                    mutations.push(format!("{}/{}", base_word, last));
                    mutations.push(format!("{}{}", base_word, last));
                }
            }
            
            // Common extensions if pattern suggests it
            if pattern.contains('.') {
                let ext = pattern.split('.').last().unwrap_or("");
                if !ext.is_empty() && ext.len() < 5 {
                    mutations.push(format!("{}.{}", base_word, ext));
                }
            }
        }
        
        // Add backup/old file patterns if they've been successful
        if self.has_successful_pattern("backup") {
            mutations.push(format!("{}.bak", base_word));
            mutations.push(format!("{}.old", base_word));
            mutations.push(format!("{}~", base_word));
        }
        
        // Add API versioning if detected
        if self.has_tech_fingerprint("api") {
            mutations.push(format!("{}/v1", base_word));
            mutations.push(format!("{}/v2", base_word));
        }
        
        mutations
    }
    
    /// Prioritize wordlist based on learned patterns
    pub fn prioritize_wordlist(&self, words: &[String]) -> Vec<(String, f32)> {
        words
            .iter()
            .map(|word| {
                let confidence = self.calculate_word_confidence(word);
                (word.clone(), confidence)
            })
            .collect()
    }
    
    /// Calculate confidence score for a word based on patterns
    pub fn calculate_word_confidence(&self, word: &str) -> f32 {
        let mut confidence = 0.5; // Base confidence
        
        // Check against successful patterns
        for pattern in self.successful_patterns.iter() {
            if word.contains(pattern.key()) || pattern.key().contains(word) {
                confidence += pattern.value().confidence * 0.2;
            }
        }
        
        // Reduce confidence for failing patterns
        for failing in self.failing_patterns.iter() {
            if word.starts_with(failing.key()) || word.ends_with(failing.key()) {
                confidence -= 0.1 * (failing.value().min(&10) / 10) as f32;
            }
        }
        
        // Boost confidence based on tech fingerprints
        let techs = self.tech_fingerprints.read();
        for tech in techs.iter() {
            if word.to_lowercase().contains(&tech.to_lowercase()) {
                confidence += 0.15;
            }
        }
        
        confidence.clamp(0.0, 1.0)
    }
    
    /// Get adaptive rate limit based on response patterns
    pub fn get_adaptive_rate(&self, max_rate: u64) -> u64 {
        let stats = self.stats.read();
        
        // If we're discovering a lot, slow down to be thorough
        if stats.discoveries > 20 && stats.total_requests > 0 {
            let discovery_rate = stats.discoveries as f64 / stats.total_requests as f64;
            if discovery_rate > 0.1 {
                return max_rate / 2; // Slow down when finding lots
            }
        }
        
        // If responses are fast, we can go faster
        if stats.avg_response_time < 100.0 {
            return max_rate;
        }
        
        // If responses are slow, reduce rate
        if stats.avg_response_time > 1000.0 {
            return max_rate / 3;
        }
        
        max_rate * 2 / 3
    }
    
    /// Get current statistics
    pub fn get_stats(&self) -> RequestStats {
        self.stats.read().clone()
    }
    
    /// Get excluded status codes
    pub fn get_excluded_codes(&self) -> Vec<u16> {
        self.excluded_codes.read().iter().copied().collect()
    }
    
    /// Get wildcard count
    pub fn get_wildcard_count(&self) -> usize {
        self.wildcard_signatures.read().len()
    }
    
    /// Get mutation count
    pub fn get_mutation_count(&self) -> usize {
        self.successful_patterns
            .iter()
            .map(|p| p.value().mutations.len())
            .sum()
    }
    
    /// Update response time statistics
    pub fn update_response_time(&self, duration_ms: f64) {
        let mut stats = self.stats.write();
        stats.avg_response_time = (stats.avg_response_time * (stats.total_requests - 1) as f64
            + duration_ms)
            / stats.total_requests as f64;
    }
    
    // Private helper methods
    
    fn generate_signature(&self, size: usize, body: &str) -> String {
        use sha2::{Sha256, Digest};
        
        let content_sample = if body.len() > 500 {
            &body[..500]
        } else {
            body
        };
        
        let mut hasher = Sha256::new();
        hasher.update(format!("{}:{}", size, content_sample));
        format!("{:x}", hasher.finalize())
    }
    
    fn is_likely_wildcard(&self, status: u16, signature: &str) -> bool {
        if let Some(pattern) = self.status_patterns.get(&status) {
            // If we've seen this status code many times with similar signature
            if pattern.count > 10 {
                let signatures = self.wildcard_signatures.read();
                // Check if similar signature exists
                return signatures.contains(signature);
            }
        }
        false
    }
    
    fn is_successful_status(&self, status: u16) -> bool {
        // Only consider 2xx successful and 301/302 redirects as interesting
        // Everything else (including 401, 403) should be filtered
        matches!(status, 200..=299 | 301 | 302)
    }
    
    fn extract_tech_fingerprints(&self, headers: &HashMap<String, String>, body: &str) {
        let mut techs = self.tech_fingerprints.write();
        
        // Check headers for technology indicators
        for (key, value) in headers {
            let key_lower = key.to_lowercase();
            let value_lower = value.to_lowercase();
            
            if key_lower == "server" || key_lower == "x-powered-by" {
                if value_lower.contains("nginx") {
                    techs.insert("nginx".to_string());
                }
                if value_lower.contains("apache") {
                    techs.insert("apache".to_string());
                }
                if value_lower.contains("php") {
                    techs.insert("php".to_string());
                }
            }
        }
        
        // Check body for framework indicators
        let body_lower = body.to_lowercase();
        if body_lower.contains("wordpress") {
            techs.insert("wordpress".to_string());
        }
        if body_lower.contains("laravel") {
            techs.insert("laravel".to_string());
        }
        if body_lower.contains("react") {
            techs.insert("react".to_string());
        }
        if body_lower.contains("/api/") || body_lower.contains("\"api\"") {
            techs.insert("api".to_string());
        }
    }
    
    fn learn_from_success(&self, path: &str) {
        // Extract pattern components
        let components: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        
        for (i, component) in components.iter().enumerate() {
            self.successful_patterns
                .entry(component.to_string())
                .and_modify(|score| {
                    score.hits += 1;
                    score.confidence = score.hits as f32 / (score.hits + score.misses) as f32;
                })
                .or_insert(PatternScore {
                    hits: 1,
                    misses: 0,
                    confidence: 1.0,
                    mutations: Vec::new(),
                });
            
            // Learn naming patterns (e.g., if we see "api_v1", learn that pattern)
            if i < components.len() - 1 {
                let pattern = format!("{}/{}", component, components[i + 1]);
                self.naming_patterns.write().push(pattern);
            }
        }
    }
    
    fn learn_from_failure(&self, path: &str) {
        // Track failing patterns to deprioritize similar words
        if let Some(first_component) = path.split('/').nth(1) {
            if !first_component.is_empty() {
                self.failing_patterns
                    .entry(first_component.to_string())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
        
        // Update miss count for patterns
        for component in path.split('/').filter(|s| !s.is_empty()) {
            if let Some(mut score) = self.successful_patterns.get_mut(component) {
                score.misses += 1;
                score.confidence = score.hits as f32 / (score.hits + score.misses) as f32;
            }
        }
    }
    
    fn extract_successful_patterns(&self) -> Vec<String> {
        let mut patterns: Vec<_> = self
            .successful_patterns
            .iter()
            .filter(|entry| entry.value().confidence > 0.5)
            .map(|entry| entry.key().clone())
            .collect();
        
        patterns.sort_by(|a, b| {
            let conf_a = self.successful_patterns.get(a).map(|s| s.confidence).unwrap_or(0.0);
            let conf_b = self.successful_patterns.get(b).map(|s| s.confidence).unwrap_or(0.0);
            conf_b.partial_cmp(&conf_a).unwrap()
        });
        
        patterns
    }
    
    fn has_successful_pattern(&self, pattern: &str) -> bool {
        self.successful_patterns
            .iter()
            .any(|entry| entry.key().contains(pattern))
    }
    
    fn has_tech_fingerprint(&self, tech: &str) -> bool {
        self.tech_fingerprints
            .read()
            .iter()
            .any(|t| t.to_lowercase().contains(&tech.to_lowercase()))
    }
}
