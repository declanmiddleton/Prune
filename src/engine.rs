use anyhow::Result;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

use crate::intelligence::IntelligenceEngine;
use crate::scanner::DirectoryScanner;
use crate::subdomain::SubdomainEnumerator;
use crate::session::{ScanConfig, ScanMode, DiscoveryResult};
use crate::ui;

/// Main discovery engine that coordinates all scanning activities
pub struct DiscoveryEngine {
    target: String,
    config: ScanConfig,
    mode: ScanMode,
    intelligence: Arc<IntelligenceEngine>,
    http_client: Client,
    results: Arc<parking_lot::RwLock<Vec<DiscoveryResult>>>,
}

impl DiscoveryEngine {
    pub async fn new(target: String, config: ScanConfig) -> Result<Self> {
        let intelligence = Arc::new(IntelligenceEngine::new());
        
        let http_client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .redirect(reqwest::redirect::Policy::limited(3))
            .user_agent("Prune/1.0 (Adaptive Discovery Engine)")
            .danger_accept_invalid_certs(true) // For pentest scenarios
            .build()?;
        
        Ok(Self {
            target,
            config,
            mode: ScanMode::Directory,
            intelligence,
            http_client,
            results: Arc::new(parking_lot::RwLock::new(Vec::new())),
        })
    }
    
    pub fn set_mode(&mut self, mode: ScanMode) {
        self.mode = mode;
    }
    
    /// Run directory discovery
    pub async fn run_directory_discovery(&mut self) -> Result<()> {
        ui::print_adaptation("Initializing adaptive directory scanner...");
        
        let scanner = DirectoryScanner::new(
            self.target.clone(),
            self.config.clone(),
            Arc::clone(&self.intelligence),
            self.http_client.clone(),
        )?;
        
        let results = scanner.scan().await?;
        
        // Store results
        let mut stored_results = self.results.write();
        stored_results.extend(results);
        
        Ok(())
    }
    
    /// Run subdomain enumeration
    pub async fn run_subdomain_discovery(&mut self) -> Result<()> {
        ui::print_adaptation("Initializing adaptive subdomain enumerator...");
        
        let enumerator = SubdomainEnumerator::new(
            self.target.clone(),
            self.config.clone(),
            Arc::clone(&self.intelligence),
            self.http_client.clone(),
        )?;
        
        let results = enumerator.enumerate().await?;
        
        // Store results
        let mut stored_results = self.results.write();
        stored_results.extend(results);
        
        Ok(())
    }
    
    /// Run combined directory and subdomain discovery
    pub async fn run_combined_discovery(&mut self) -> Result<()> {
        ui::print_adaptation("Initializing combined discovery mode...");
        
        // Phase 1: Quick subdomain enumeration to find targets
        ui::print_section_header("Phase 1: Subdomain Discovery");
        let enumerator = SubdomainEnumerator::new(
            self.target.clone(),
            self.config.clone(),
            Arc::clone(&self.intelligence),
            self.http_client.clone(),
        )?;
        
        let subdomain_results = enumerator.enumerate().await?;
        
        // Store subdomain results
        {
            let mut stored_results = self.results.write();
            stored_results.extend(subdomain_results.clone());
        }
        
        sleep(Duration::from_millis(500)).await;
        
        // Phase 2: Directory discovery on main target
        ui::print_section_header("Phase 2: Directory Discovery on Primary Target");
        let scanner = DirectoryScanner::new(
            self.target.clone(),
            self.config.clone(),
            Arc::clone(&self.intelligence),
            self.http_client.clone(),
        )?;
        
        let dir_results = scanner.scan().await?;
        
        // Store directory results
        {
            let mut stored_results = self.results.write();
            stored_results.extend(dir_results);
        }
        
        // Phase 3: Optional directory discovery on discovered subdomains
        if !subdomain_results.is_empty() && subdomain_results.len() <= 5 {
            ui::print_section_header("Phase 3: Directory Discovery on Subdomains");
            
            for subdomain_result in subdomain_results.iter().take(5) {
                ui::print_adaptation(&format!("Scanning subdomain: {}", subdomain_result.url));
                
                let subdomain_scanner = DirectoryScanner::new(
                    subdomain_result.url.clone(),
                    self.config.clone(),
                    Arc::clone(&self.intelligence),
                    self.http_client.clone(),
                )?;
                
                if let Ok(results) = subdomain_scanner.scan().await {
                    let mut stored_results = self.results.write();
                    stored_results.extend(results);
                }
                
                sleep(Duration::from_millis(200)).await;
            }
        }
        
        Ok(())
    }
    
    /// Resume from previous results
    pub fn resume_from_results(&mut self, previous_results: Vec<DiscoveryResult>) {
        let mut results = self.results.write();
        *results = previous_results;
    }
    
    /// Get all results
    pub fn get_results(&self) -> Vec<DiscoveryResult> {
        self.results.read().clone()
    }
}
