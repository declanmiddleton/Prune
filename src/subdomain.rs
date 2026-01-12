use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use futures::stream::{self, StreamExt};
use chrono::Utc;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};

use crate::intelligence::IntelligenceEngine;
use crate::session::{ScanConfig, DiscoveryResult};
use crate::wordlist::WordlistManager;
use crate::ui;

/// Adaptive subdomain enumerator with intelligent pattern learning
pub struct SubdomainEnumerator {
    domain: String,
    config: ScanConfig,
    intelligence: Arc<IntelligenceEngine>,
    http_client: Client,
    dns_resolver: TokioAsyncResolver,
    wordlist_manager: WordlistManager,
}

impl SubdomainEnumerator {
    pub fn new(
        domain: String,
        config: ScanConfig,
        intelligence: Arc<IntelligenceEngine>,
        http_client: Client,
    ) -> Result<Self> {
        let wordlist_manager = WordlistManager::new()?;
        
        // Create DNS resolver
        let dns_resolver = TokioAsyncResolver::tokio(
            ResolverConfig::default(),
            ResolverOpts::default(),
        );
        
        // Extract root domain from URL if needed
        let clean_domain = Self::extract_domain(&domain)?;
        
        Ok(Self {
            domain: clean_domain,
            config,
            intelligence,
            http_client,
            dns_resolver,
            wordlist_manager,
        })
    }
    
    /// Run adaptive subdomain enumeration
    pub async fn enumerate(&self) -> Result<Vec<DiscoveryResult>> {
        let mut results = Vec::new();
        
        ui::print_info("Loading subdomain wordlist...");
        
        // Load and prioritize wordlist
        let wordlist = self.wordlist_manager.load_subdomain_wordlist()?;
        let mut prioritized = self.intelligence.prioritize_wordlist(&wordlist);
        
        // Sort by confidence
        prioritized.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        ui::print_adaptation(&format!("Loaded {} subdomains, prioritized by confidence", prioritized.len()));
        
        // Create rate limiter (more conservative for DNS)
        let rate_limit = (self.intelligence.get_adaptive_rate(self.config.rate_limit) / 2).max(20);
        let semaphore = Arc::new(Semaphore::new(rate_limit as usize));
        
        ui::print_info(&format!("Starting enumeration with rate: {} req/s", rate_limit));
        
        let total_words = prioritized.len();
        let mut processed = 0;
        let mut discoveries = 0;
        let start_time = Instant::now();
        
        // Process in chunks for adaptive behavior
        let chunk_size = 30;
        for chunk in prioritized.chunks(chunk_size) {
            let mut current_batch: Vec<(String, f32)> = chunk.to_vec();
            
            // Generate mutations from discovered patterns
            if discoveries > 3 && processed > 50 {
                let mutations = self.generate_subdomain_mutations(&results);
                for mutation in mutations.iter().take(5) {
                    let confidence = self.intelligence.calculate_word_confidence(mutation);
                    current_batch.push((mutation.clone(), confidence));
                }
                
                if !mutations.is_empty() {
                    ui::print_adaptation(&format!(
                        "Generated {} subdomain mutations from patterns",
                        mutations.len().min(5)
                    ));
                }
            }
            
            // Process batch concurrently
            let batch_results: Vec<Option<DiscoveryResult>> = stream::iter(current_batch)
                .map(|(subdomain, confidence)| {
                    let sem = Arc::clone(&semaphore);
                    let intel = Arc::clone(&self.intelligence);
                    let client = self.http_client.clone();
                    let resolver = self.dns_resolver.clone();
                    let domain = self.domain.clone();
                    
                    async move {
                        let _permit = sem.acquire().await.ok()?;
                        
                        let full_domain = format!("{}.{}", subdomain, domain);
                        
                        // First, check DNS resolution
                        let start = Instant::now();
                        let lookup_result = resolver.lookup_ip(full_domain.clone()).await;
                        
                        if lookup_result.is_err() {
                            return None;
                        }
                        
                        // DNS resolved, now try HTTP/HTTPS
                        let https_url = format!("https://{}", full_domain);
                        let http_url = format!("http://{}", full_domain);
                        
                        // Try HTTPS first
                        let response = if let Ok(resp) = client.get(&https_url)
                            .timeout(Duration::from_secs(5))
                            .send()
                            .await
                        {
                            Some((resp, https_url))
                        } else if let Ok(resp) = client.get(&http_url)
                            .timeout(Duration::from_secs(5))
                            .send()
                            .await
                        {
                            Some((resp, http_url))
                        } else {
                            None
                        };
                        
                        if let Some((resp, url)) = response {
                            let duration = start.elapsed().as_millis() as f64;
                            intel.update_response_time(duration);
                            
                            let status = resp.status().as_u16();
                            let headers = extract_headers(&resp);
                            let size = resp.content_length().unwrap_or(0) as usize;
                            let body = resp.text().await.unwrap_or_default();
                            
                            // Learn from this response
                            intel.learn_from_response(status, size, &subdomain, &body, &headers);
                            
                            // Filter based on learned patterns
                            if intel.should_filter(status, size, &body) {
                                return None;
                            }
                            
                            Some(DiscoveryResult {
                                url,
                                status_code: status,
                                size,
                                confidence,
                                discovered_at: Utc::now(),
                            })
                        } else {
                            None
                        }
                    }
                })
                .buffer_unordered(rate_limit as usize)
                .collect()
                .await;
            
            // Collect and display results
            for result in batch_results.into_iter().flatten() {
                ui::print_finding(
                    result.status_code,
                    &result.url,
                    result.size,
                    result.confidence,
                );
                discoveries += 1;
                results.push(result);
            }
            
            processed += chunk.len();
            
            // Update progress
            let elapsed = start_time.elapsed().as_secs_f32();
            let rate = processed as f32 / elapsed;
            ui::print_progress(processed, total_words, rate, discoveries);
            
            // Adaptive delay
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        
        ui::clear_line();
        
        // Display intelligence summary
        ui::print_intelligence_summary(
            &self.intelligence.get_excluded_codes(),
            self.intelligence.get_wildcard_count(),
            self.intelligence.get_mutation_count(),
            self.calculate_overall_confidence(&results),
        );
        
        Ok(results)
    }
    
    /// Generate subdomain mutations from discovered patterns
    fn generate_subdomain_mutations(&self, results: &[DiscoveryResult]) -> Vec<String> {
        let mut mutations = Vec::new();
        
        for result in results.iter().rev().take(5) {
            if let Ok(parsed) = url::Url::parse(&result.url) {
                if let Some(host) = parsed.host_str() {
                    // Extract subdomain part
                    let parts: Vec<&str> = host.split('.').collect();
                    if parts.len() > 2 {
                        let subdomain = parts[0];
                        
                        // Common patterns
                        if !subdomain.contains('-') {
                            mutations.push(format!("{}-api", subdomain));
                            mutations.push(format!("{}-dev", subdomain));
                            mutations.push(format!("{}-staging", subdomain));
                            mutations.push(format!("{}-prod", subdomain));
                            mutations.push(format!("{}2", subdomain));
                            mutations.push(format!("new-{}", subdomain));
                            mutations.push(format!("old-{}", subdomain));
                        }
                        
                        // Number variations
                        if subdomain.chars().last().map(|c| c.is_numeric()).unwrap_or(false) {
                            let base = subdomain.trim_end_matches(char::is_numeric);
                            for i in 1..=5 {
                                mutations.push(format!("{}{}", base, i));
                            }
                        }
                    }
                }
            }
        }
        
        // Deduplicate
        mutations.sort();
        mutations.dedup();
        
        mutations
    }
    
    /// Calculate overall confidence score
    fn calculate_overall_confidence(&self, results: &[DiscoveryResult]) -> f32 {
        if results.is_empty() {
            return 0.0;
        }
        
        let sum: f32 = results.iter().map(|r| r.confidence).sum();
        sum / results.len() as f32
    }
    
    /// Extract clean domain from URL or domain string
    fn extract_domain(input: &str) -> Result<String> {
        // Try to parse as URL first
        if let Ok(parsed) = url::Url::parse(input) {
            if let Some(host) = parsed.host_str() {
                return Ok(host.to_string());
            }
        }
        
        // Try to parse with http:// prefix
        if let Ok(parsed) = url::Url::parse(&format!("http://{}", input)) {
            if let Some(host) = parsed.host_str() {
                return Ok(host.to_string());
            }
        }
        
        // Just use as-is
        Ok(input.to_string())
    }
}

/// Extract headers from response
fn extract_headers(response: &reqwest::Response) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    
    for (key, value) in response.headers() {
        if let Ok(val_str) = value.to_str() {
            headers.insert(key.to_string(), val_str.to_string());
        }
    }
    
    headers
}
