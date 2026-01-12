use clap::{Parser, Subcommand, ValueEnum};
use anyhow::Result;
use colored::Colorize;
use std::io::{self, Write};

use crate::engine::DiscoveryEngine;
use crate::session::{SessionManager, ScanMode};
use crate::ui;

#[derive(Parser)]
#[command(name = "prune")]
#[command(about = "Adaptive discovery tool for directories and subdomains", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run interactive discovery with mode selection
    Scan { url: String },
    
    /// Run adaptive directory discovery only
    Dir { url: String },
    
    /// Run adaptive subdomain enumeration only
    Sub { domain: String },
    
    /// Run both directory and subdomain discovery together
    Both { url: String },
    
    /// Launch graphical user interface
    Gui,
    
    /// Configure crawling behavior
    Crawl {
        #[arg(value_enum)]
        mode: CrawlMode,
    },
    
    /// Manually exclude specific status codes
    Status {
        #[arg(value_delimiter = ',')]
        exclude: Vec<u16>,
    },
    
    /// Set request pacing rate
    Rate {
        #[arg(value_enum)]
        speed: RateSpeed,
    },
    
    /// Resume last discovery session
    Resume,
    
    /// Show filtered unique findings
    Results,
}

#[derive(Clone, ValueEnum)]
pub enum CrawlMode {
    On,
    Off,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum RateSpeed {
    Slow,
    Normal,
    Fast,
}

/// Handle interactive scan with mode selection
pub async fn handle_scan(url: String) -> Result<()> {
    ui::print_section_header("Interactive Discovery");
    
    println!("\n{}", "Select discovery mode:".color(ui::PRIMARY_COLOR));
    println!("  {} Directory discovery only", "1.".color(ui::SECONDARY_COLOR));
    println!("  {} Subdomain enumeration only", "2.".color(ui::SECONDARY_COLOR));
    println!("  {} Both (combined intelligence)", "3.".color(ui::SECONDARY_COLOR));
    
    print!("\n{} ", "→".color(ui::PRIMARY_COLOR));
    io::stdout().flush()?;
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    
    match choice.trim() {
        "1" => handle_directory_scan(url).await?,
        "2" => {
            let domain = extract_domain(&url)?;
            handle_subdomain_scan(domain).await?
        }
        "3" => handle_both(url).await?,
        _ => {
            println!("{}", "Invalid choice. Defaulting to directory discovery.".color(ui::WARNING_COLOR));
            handle_directory_scan(url).await?
        }
    }
    
    Ok(())
}

/// Handle directory-only scanning
pub async fn handle_directory_scan(url: String) -> Result<()> {
    ui::print_section_header("Directory Discovery");
    
    let session_mgr = SessionManager::new()?;
    let config = session_mgr.load_config()?;
    
    let mut engine = DiscoveryEngine::new(url.clone(), config).await?;
    engine.set_mode(ScanMode::Directory);
    
    let session_id = session_mgr.create_session(&url, ScanMode::Directory)?;
    
    ui::print_info(&format!("Target: {}", url));
    ui::print_info(&format!("Session: {}", session_id));
    
    engine.run_directory_discovery().await?;
    
    session_mgr.save_session(&session_id, &engine.get_results())?;
    
    ui::print_success("Directory discovery complete!");
    
    Ok(())
}

/// Handle subdomain-only scanning
pub async fn handle_subdomain_scan(domain: String) -> Result<()> {
    ui::print_section_header("Subdomain Enumeration");
    
    let session_mgr = SessionManager::new()?;
    let config = session_mgr.load_config()?;
    
    let mut engine = DiscoveryEngine::new(domain.clone(), config).await?;
    engine.set_mode(ScanMode::Subdomain);
    
    let session_id = session_mgr.create_session(&domain, ScanMode::Subdomain)?;
    
    ui::print_info(&format!("Domain: {}", domain));
    ui::print_info(&format!("Session: {}", session_id));
    
    engine.run_subdomain_discovery().await?;
    
    session_mgr.save_session(&session_id, &engine.get_results())?;
    
    ui::print_success("Subdomain enumeration complete!");
    
    Ok(())
}

/// Handle combined directory and subdomain scanning
pub async fn handle_both(url: String) -> Result<()> {
    ui::print_section_header("Combined Discovery");
    
    let session_mgr = SessionManager::new()?;
    let config = session_mgr.load_config()?;
    
    let mut engine = DiscoveryEngine::new(url.clone(), config).await?;
    engine.set_mode(ScanMode::Both);
    
    let session_id = session_mgr.create_session(&url, ScanMode::Both)?;
    
    ui::print_info(&format!("Target: {}", url));
    ui::print_info(&format!("Session: {}", session_id));
    
    engine.run_combined_discovery().await?;
    
    session_mgr.save_session(&session_id, &engine.get_results())?;
    
    ui::print_success("Combined discovery complete!");
    
    Ok(())
}

/// Handle crawl configuration
pub fn handle_crawl_config(mode: CrawlMode) -> Result<()> {
    let session_mgr = SessionManager::new()?;
    let mut config = session_mgr.load_config()?;
    
    match mode {
        CrawlMode::On => {
            config.crawling_enabled = true;
            ui::print_success("Crawling enabled");
        }
        CrawlMode::Off => {
            config.crawling_enabled = false;
            ui::print_success("Crawling disabled");
        }
    }
    
    session_mgr.save_config(&config)?;
    
    Ok(())
}

/// Handle status code exclusion configuration
pub fn handle_status_exclude(codes: Vec<u16>) -> Result<()> {
    let session_mgr = SessionManager::new()?;
    let mut config = session_mgr.load_config()?;
    
    config.excluded_status_codes.extend(codes.clone());
    session_mgr.save_config(&config)?;
    
    ui::print_success(&format!("Excluded status codes: {:?}", codes));
    
    Ok(())
}

/// Handle rate configuration
pub fn handle_rate_config(speed: RateSpeed) -> Result<()> {
    let session_mgr = SessionManager::new()?;
    let mut config = session_mgr.load_config()?;
    
    config.rate_limit = match speed {
        RateSpeed::Slow => 50,
        RateSpeed::Normal => 100,
        RateSpeed::Fast => 200,
    };
    
    session_mgr.save_config(&config)?;
    
    ui::print_success(&format!("Rate limit set to: {:?}", speed));
    
    Ok(())
}

/// Handle session resume
pub async fn handle_resume() -> Result<()> {
    ui::print_section_header("Resume Session");
    
    let session_mgr = SessionManager::new()?;
    let last_session = session_mgr.get_last_session()?;
    
    if let Some((session_id, session_data)) = last_session {
        ui::print_info(&format!("Resuming session: {}", session_id));
        ui::print_info(&format!("Target: {}", session_data.target));
        ui::print_info(&format!("Mode: {:?}", session_data.mode));
        
        let config = session_mgr.load_config()?;
        let mut engine = DiscoveryEngine::new(session_data.target.clone(), config).await?;
        engine.set_mode(session_data.mode);
        engine.resume_from_results(session_data.results);
        
        match session_data.mode {
            ScanMode::Directory => engine.run_directory_discovery().await?,
            ScanMode::Subdomain => engine.run_subdomain_discovery().await?,
            ScanMode::Both => engine.run_combined_discovery().await?,
        }
        
        session_mgr.save_session(&session_id, &engine.get_results())?;
        
        ui::print_success("Session resumed and completed!");
    } else {
        ui::print_warning("No previous session found");
    }
    
    Ok(())
}

/// Handle results display
pub fn handle_results() -> Result<()> {
    ui::print_section_header("Discovery Results");
    
    let session_mgr = SessionManager::new()?;
    let last_session = session_mgr.get_last_session()?;
    
    if let Some((session_id, session_data)) = last_session {
        ui::print_info(&format!("Session: {}", session_id));
        ui::print_info(&format!("Target: {}", session_data.target));
        ui::print_info(&format!("Mode: {:?}", session_data.mode));
        
        println!("\n{}", "═".repeat(60).color(ui::SECONDARY_COLOR));
        
        for result in &session_data.results {
            let status_color = match result.status_code {
                200..=299 => ui::SUCCESS_COLOR,
                300..=399 => ui::PRIMARY_COLOR,
                400..=499 => ui::WARNING_COLOR,
                _ => ui::ERROR_COLOR,
            };
            
            println!(
                "{} {} {} {}",
                result.status_code.to_string().color(status_color),
                "│".color(ui::SECONDARY_COLOR),
                result.url.color(ui::PRIMARY_COLOR),
                if result.size > 0 {
                    format!("({})", format_size(result.size)).color(ui::SECONDARY_COLOR)
                } else {
                    "".color(ui::SECONDARY_COLOR)
                }
            );
        }
        
        println!("{}", "═".repeat(60).color(ui::SECONDARY_COLOR));
        println!("\n{} {}", "Total findings:".color(ui::SECONDARY_COLOR), session_data.results.len().to_string().color(ui::PRIMARY_COLOR));
    } else {
        ui::print_warning("No results found. Run a scan first.");
    }
    
    Ok(())
}

/// Extract domain from URL
fn extract_domain(url: &str) -> Result<String> {
    let parsed = url::Url::parse(url)
        .or_else(|_| url::Url::parse(&format!("http://{}", url)))?;
    
    Ok(parsed.host_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid URL: no host"))?
        .to_string())
}

/// Format size in human-readable format
fn format_size(bytes: usize) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_idx])
}
