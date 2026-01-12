mod cli;
mod engine;
mod scanner;
mod subdomain;
mod crawler;
mod ui;
mod wordlist;
mod session;
mod intelligence;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    // Display banner
    ui::display_banner();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Scan { url } => {
            cli::handle_scan(url).await?;
        }
        Commands::Dir { url } => {
            cli::handle_directory_scan(url).await?;
        }
        Commands::Sub { domain } => {
            cli::handle_subdomain_scan(domain).await?;
        }
        Commands::Both { url } => {
            cli::handle_both(url).await?;
        }
        Commands::Crawl { mode } => {
            cli::handle_crawl_config(mode)?;
        }
        Commands::Status { exclude } => {
            cli::handle_status_exclude(exclude)?;
        }
        Commands::Rate { speed } => {
            cli::handle_rate_config(speed)?;
        }
        Commands::Resume => {
            cli::handle_resume().await?;
        }
        Commands::Results => {
            cli::handle_results()?;
        }
    }
    
    Ok(())
}
