use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use futures::stream::{self, StreamExt};
use chrono::Utc;
use parking_lot::RwLock;

use crate::intelligence::IntelligenceEngine;
use crate::session::{ScanConfig, DiscoveryResult};
use crate::ui;

/// Passive crawler that extracts links and paths from responses
pub struct Crawler {
    base_url: String,
    config: ScanConfig,
    intelligence: Arc<IntelligenceEngine>,
    http_client: Client,
    visited: Arc<RwLock<HashSet<String>>>,
}

impl Crawler {
    pub fn new(
        base_url: String,
        config: ScanConfig,
        intelligence: Arc<IntelligenceEngine>,
        http_client: Client,
    ) -> Self {
        Self {
            base_url,
            config,
            intelligence,
            http_client,
            visited: Arc::new(RwLock::new(HashSet::new())),
        }
    }
    
    /// Crawl a list of target URLs and extract additional paths
    pub async fn crawl_targets(&self, targets: Vec<String>) -> Result<Vec<DiscoveryResult>> {
        let mut results = Vec::new();
        
        if targets.is_empty() {
            return Ok(results);
        }
        
        ui::print_info(&format!("Crawling {} targets for additional paths...", targets.len()));
        
        // Extract links from each target
        let mut discovered_urls = HashSet::new();
        
        for target in targets.iter().take(10) {
            if let Ok(links) = self.extract_links_from_page(target).await {
                discovered_urls.extend(links);
            }
            
            // Rate limiting for crawling
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
        
        ui::print_adaptation(&format!("Extracted {} unique URLs from crawling", discovered_urls.len()));
        
        // Validate discovered URLs
        if discovered_urls.is_empty() {
            return Ok(results);
        }
        
        // Test discovered URLs (with strict rate limiting)
        let rate_limit = 20; // Conservative rate for crawling
        let semaphore = Arc::new(Semaphore::new(rate_limit));
        
        let validated: Vec<DiscoveryResult> = stream::iter(discovered_urls)
            .map(|url| {
                let sem = Arc::clone(&semaphore);
                let intel = Arc::clone(&self.intelligence);
                let client = self.http_client.clone();
                
                async move {
                    let _permit = sem.acquire().await.ok()?;
                    
                    let response = match client.get(&url)
                        .timeout(Duration::from_secs(5))
                        .send()
                        .await
                    {
                        Ok(resp) => resp,
                        Err(_) => return None,
                    };
                    
                    let status = response.status().as_u16();
                    let size = response.content_length().unwrap_or(0) as usize;
                    let body = response.text().await.unwrap_or_default();
                    
                    // Filter using intelligence
                    if intel.should_filter(status, size, &body) {
                        return None;
                    }
                    
                    Some(DiscoveryResult {
                        url: url.clone(),
                        status_code: status,
                        size,
                        confidence: 0.6, // Moderate confidence for crawled findings
                        discovered_at: Utc::now(),
                    })
                }
            })
            .buffer_unordered(rate_limit)
            .filter_map(|x| async { x })
            .collect()
            .await;
        
        for result in &validated {
            ui::print_finding(result.status_code, &result.url, result.size, result.confidence);
        }
        
        results.extend(validated);
        
        Ok(results)
    }
    
    /// Extract links and paths from a single page
    async fn extract_links_from_page(&self, url: &str) -> Result<Vec<String>> {
        let mut links = Vec::new();
        
        // Check if already visited
        {
            let visited = self.visited.read();
            if visited.contains(url) {
                return Ok(links);
            }
        }
        
        // Mark as visited
        self.visited.write().insert(url.to_string());
        
        // Fetch page
        let response = self.http_client
            .get(url)
            .timeout(Duration::from_secs(self.config.timeout_seconds))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Ok(links);
        }
        
        let body = response.text().await?;
        let document = Html::parse_document(&body);
        
        // Extract from various sources
        links.extend(self.extract_from_links(&document)?);
        links.extend(self.extract_from_scripts(&document)?);
        links.extend(self.extract_from_forms(&document)?);
        links.extend(self.extract_from_comments(&body));
        
        // Normalize and filter links
        let normalized: Vec<String> = links
            .into_iter()
            .filter_map(|link| self.normalize_url(&link).ok())
            .filter(|link| self.is_in_scope(link))
            .collect();
        
        Ok(normalized)
    }
    
    /// Extract links from <a> tags
    fn extract_from_links(&self, document: &Html) -> Result<Vec<String>> {
        let mut links = Vec::new();
        
        let selector = Selector::parse("a[href]").unwrap();
        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                links.push(href.to_string());
            }
        }
        
        Ok(links)
    }
    
    /// Extract links from <script> src attributes
    fn extract_from_scripts(&self, document: &Html) -> Result<Vec<String>> {
        let mut links = Vec::new();
        
        let selector = Selector::parse("script[src]").unwrap();
        for element in document.select(&selector) {
            if let Some(src) = element.value().attr("src") {
                links.push(src.to_string());
            }
        }
        
        // Also extract from inline scripts (look for URL patterns)
        let script_selector = Selector::parse("script").unwrap();
        for element in document.select(&script_selector) {
            let text = element.text().collect::<Vec<_>>().join(" ");
            links.extend(self.extract_urls_from_text(&text));
        }
        
        Ok(links)
    }
    
    /// Extract from forms
    fn extract_from_forms(&self, document: &Html) -> Result<Vec<String>> {
        let mut links = Vec::new();
        
        let selector = Selector::parse("form[action]").unwrap();
        for element in document.select(&selector) {
            if let Some(action) = element.value().attr("action") {
                links.push(action.to_string());
            }
        }
        
        Ok(links)
    }
    
    /// Extract URLs from HTML comments
    fn extract_from_comments(&self, body: &str) -> Vec<String> {
        let mut links = Vec::new();
        
        // Simple regex-like extraction of URLs from comments
        for line in body.lines() {
            if line.contains("<!--") {
                links.extend(self.extract_urls_from_text(line));
            }
        }
        
        links
    }
    
    /// Extract URL patterns from text
    fn extract_urls_from_text(&self, text: &str) -> Vec<String> {
        let mut urls = Vec::new();
        
        // Look for common URL patterns
        let patterns = [
            r#"["'](\/[a-zA-Z0-9._\-\/]+)["']"#,
            r#"url\s*[:=]\s*["']([^"']+)["']"#,
            r#"href\s*[:=]\s*["']([^"']+)["']"#,
        ];
        
        for pattern in &patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                for cap in re.captures_iter(text) {
                    if let Some(matched) = cap.get(1) {
                        urls.push(matched.as_str().to_string());
                    }
                }
            }
        }
        
        urls
    }
    
    /// Normalize URL to absolute form
    fn normalize_url(&self, link: &str) -> Result<String> {
        // Parse base URL
        let base = url::Url::parse(&self.base_url)?;
        
        // Join with relative URL
        let absolute = base.join(link)?;
        
        Ok(absolute.to_string())
    }
    
    /// Check if URL is in scope (same domain)
    fn is_in_scope(&self, link: &str) -> bool {
        let base = match url::Url::parse(&self.base_url) {
            Ok(u) => u,
            Err(_) => return false,
        };
        
        let target = match url::Url::parse(link) {
            Ok(u) => u,
            Err(_) => return false,
        };
        
        // Check if same host
        if base.host_str() != target.host_str() {
            return false;
        }
        
        // Exclude certain file extensions
        if let Some(segments) = target.path_segments() {
            if let Some(last) = segments.last() {
                let excluded_exts = [
                    ".jpg", ".jpeg", ".png", ".gif", ".svg", ".ico",
                    ".woff", ".woff2", ".ttf", ".eot",
                    ".mp4", ".webm", ".mp3", ".wav",
                    ".pdf", ".zip", ".tar", ".gz",
                ];
                
                for ext in &excluded_exts {
                    if last.to_lowercase().ends_with(ext) {
                        return false;
                    }
                }
            }
        }
        
        true
    }
}
