use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use futures::stream::{self, StreamExt};
use chrono::Utc;

use crate::intelligence::IntelligenceEngine;
use crate::session::{ScanConfig, DiscoveryResult};
use crate::wordlist::WordlistManager;
use crate::crawler::Crawler;
use crate::ui;

/// Adaptive directory scanner that learns from responses
pub struct DirectoryScanner {
    target: String,
    config: ScanConfig,
    intelligence: Arc<IntelligenceEngine>,
    http_client: Client,
    wordlist_manager: WordlistManager,
}

impl DirectoryScanner {
    pub fn new(
        target: String,
        config: ScanConfig,
        intelligence: Arc<IntelligenceEngine>,
        http_client: Client,
    ) -> Result<Self> {
        let wordlist_manager = WordlistManager::new()?;
        
        Ok(Self {
            target,
            config,
            intelligence,
            http_client,
            wordlist_manager,
        })
    }
    
    /// Run the adaptive directory scan
    pub async fn scan(&self) -> Result<Vec<DiscoveryResult>> {
        let mut results = Vec::new();
        
        // Ensure target has proper scheme
        let base_url = if self.target.starts_with("http://") || self.target.starts_with("https://") {
            self.target.clone()
        } else {
            format!("https://{}", self.target)
        };
        
        ui::print_info(&format!("Loading wordlist..."));
        
        // Load and prioritize wordlist
        let wordlist = self.wordlist_manager.load_directory_wordlist()?;
        let mut prioritized = self.intelligence.prioritize_wordlist(&wordlist);
        
        // Sort by confidence (highest first)
        prioritized.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        ui::print_adaptation(&format!("Loaded {} words, prioritized by confidence", prioritized.len()));
        
        // Create rate limiter
        let rate_limit = self.intelligence.get_adaptive_rate(self.config.rate_limit);
        let semaphore = Arc::new(Semaphore::new(rate_limit as usize));
        
        ui::print_info(&format!("Starting scan with adaptive rate: {} req/s", rate_limit));
        
        let total_words = prioritized.len();
        let mut processed = 0;
        let mut discoveries = 0;
        let start_time = Instant::now();
        
        // Process wordlist in chunks to allow for adaptive behavior
        let chunk_size = 50;
        for chunk in prioritized.chunks(chunk_size) {
            // Check if we should generate mutations based on what we've learned
            let mut current_batch: Vec<(String, f32)> = chunk.to_vec();
            
            if discoveries > 5 && processed > 100 {
                // Generate intelligent mutations from successful patterns
                let mutations = self.generate_adaptive_mutations(&results);
                for mutation in mutations.iter().take(10) {
                    let confidence = self.intelligence.calculate_word_confidence(mutation);
                    current_batch.push((mutation.clone(), confidence));
                }
                
                if !mutations.is_empty() {
                    ui::print_adaptation(&format!(
                        "Generated {} adaptive mutations from successful patterns",
                        mutations.len().min(10)
                    ));
                }
            }
            
            // Process batch concurrently
            let batch_results: Vec<Option<DiscoveryResult>> = stream::iter(current_batch)
                .map(|(word, confidence)| {
                    let sem = Arc::clone(&semaphore);
                    let intel = Arc::clone(&self.intelligence);
                    let client = self.http_client.clone();
                    let url = format!("{}/{}", base_url.trim_end_matches('/'), word.trim_start_matches('/'));
                    let config = self.config.clone();
                    
                    async move {
                        let _permit = sem.acquire().await.ok()?;
                        
                        let start = Instant::now();
                        let response = match client.get(&url).send().await {
                            Ok(resp) => resp,
                            Err(_) => return None,
                        };
                        
                        let duration = start.elapsed().as_millis() as f64;
                        intel.update_response_time(duration);
                        
                        let status = response.status().as_u16();
                        let headers = extract_headers(&response);
                        let size = response.content_length().unwrap_or(0) as usize;
                        
                        // Get body for intelligence learning
                        let body = response.text().await.unwrap_or_default();
                        
                        // Learn from this response
                        intel.learn_from_response(status, size, &word, &body, &headers);
                        
                        // Filter based on learned patterns
                        if intel.should_filter(status, size, &body) {
                            return None;
                        }
                        
                        // This is a potentially interesting finding
                        Some(DiscoveryResult {
                            url: url.clone(),
                            status_code: status,
                            size,
                            confidence,
                            discovered_at: Utc::now(),
                        })
                    }
                })
                .buffer_unordered(rate_limit as usize)
                .collect()
                .await;
            
            // Collect results and display findings
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
            
            // Adaptive delay between chunks
            tokio::time::sleep(Duration::from_millis(50)).await;
            
            // Adjust rate limit based on what we're learning
            let new_rate = self.intelligence.get_adaptive_rate(self.config.rate_limit);
            if new_rate != rate_limit {
                ui::print_adaptation(&format!("Adapting rate limit: {} → {} req/s", rate_limit, new_rate));
            }
        }
        
        ui::clear_line();
        
        // Optional crawling phase
        if self.config.crawling_enabled && !results.is_empty() {
            ui::print_section_header("Passive Crawling Phase");
            results.extend(self.crawl_discovered_paths(&base_url, &results).await?);
        }
        
        // Display intelligence summary
        ui::print_intelligence_summary(
            &self.intelligence.get_excluded_codes(),
            self.intelligence.get_wildcard_count(),
            self.intelligence.get_mutation_count(),
            self.calculate_overall_confidence(&results),
        );
        
        Ok(results)
    }
    
    /// Generate adaptive mutations from successful results
    fn generate_adaptive_mutations(&self, results: &[DiscoveryResult]) -> Vec<String> {
        let mut mutations = Vec::new();
        
        for result in results.iter().rev().take(10) {
            // Extract path components
            if let Ok(parsed) = url::Url::parse(&result.url) {
                if let Some(path) = parsed.path_segments() {
                    let segments: Vec<&str> = path.collect();
                    
                    for segment in segments {
                        if !segment.is_empty() {
                            // Generate mutations using intelligence engine
                            let generated = self.intelligence.generate_mutations(segment);
                            mutations.extend(generated);
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
    
    /// Crawl discovered paths for additional targets
    async fn crawl_discovered_paths(
        &self,
        base_url: &str,
        results: &[DiscoveryResult],
    ) -> Result<Vec<DiscoveryResult>> {
        let crawler = Crawler::new(
            base_url.to_string(),
            self.config.clone(),
            Arc::clone(&self.intelligence),
            self.http_client.clone(),
        );
        
        // Only crawl successful responses
        let crawl_targets: Vec<String> = results
            .iter()
            .filter(|r| matches!(r.status_code, 200..=299))
            .map(|r| r.url.clone())
            .take(10) // Limit crawl targets
            .collect();
        
        crawler.crawl_targets(crawl_targets).await
    }
    
    /// Calculate overall confidence score
    fn calculate_overall_confidence(&self, results: &[DiscoveryResult]) -> f32 {
        if results.is_empty() {
            return 0.0;
        }
        
        let sum: f32 = results.iter().map(|r| r.confidence).sum();
        sum / results.len() as f32
    }
}

/// Extract headers from response into HashMap
fn extract_headers(response: &reqwest::Response) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    
    for (key, value) in response.headers() {
        if let Ok(val_str) = value.to_str() {
            headers.insert(key.to_string(), val_str.to_string());
        }
    }
    
    headers
}
