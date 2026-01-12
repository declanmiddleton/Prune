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
mod gui;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Check if launching GUI (no async needed)
    if matches!(cli.command, Commands::Gui) {
        // Display banner for GUI launch
        ui::display_banner();
        println!("Launching graphical interface...\n");
        return gui::launch_gui();
    }
    
    // For all other commands, use tokio runtime
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            // Display banner
            ui::display_banner();
            
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
                Commands::Gui => {
                    // Already handled above
                    unreachable!()
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
        })
}
