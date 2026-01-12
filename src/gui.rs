use eframe::egui;
use std::sync::Arc;
use parking_lot::Mutex;
use poll_promise::Promise;

use crate::engine::DiscoveryEngine;
use crate::session::{ScanConfig, ScanMode, DiscoveryResult};

/// Main GUI application structure
pub struct PruneGui {
    // Input fields
    target_url: String,
    scan_mode: ScanMode,
    
    // Configuration
    config: ScanConfig,
    custom_wordlist_path: Option<String>,
    use_custom_wordlist: bool,
    
    // UI state
    scanning: bool,
    scan_promise: Option<Promise<Result<Vec<DiscoveryResult>, String>>>,
    results: Vec<DiscoveryResult>,
    error_message: Option<String>,
    success_message: Option<String>,
    
    // Progress tracking
    progress: Arc<Mutex<ScanProgress>>,
    
    // Results filtering
    filter_text: String,
    show_only_200: bool,
}

#[derive(Clone)]
struct ScanProgress {
    current: usize,
    total: usize,
    discoveries: usize,
    rate: f32,
    status: String,
}

impl Default for ScanProgress {
    fn default() -> Self {
        Self {
            current: 0,
            total: 0,
            discoveries: 0,
            rate: 0.0,
            status: "Ready".to_string(),
        }
    }
}

impl Default for PruneGui {
    fn default() -> Self {
        Self {
            target_url: String::new(),
            scan_mode: ScanMode::Directory,
            config: ScanConfig::default(),
            custom_wordlist_path: None,
            use_custom_wordlist: false,
            scanning: false,
            scan_promise: None,
            results: Vec::new(),
            error_message: None,
            success_message: None,
            progress: Arc::new(Mutex::new(ScanProgress::default())),
            filter_text: String::new(),
            show_only_200: true,
        }
    }
}

impl PruneGui {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
    
    fn start_scan(&mut self) {
        if self.target_url.is_empty() {
            self.error_message = Some("Please enter a target URL or domain".to_string());
            return;
        }
        
        self.scanning = true;
        self.error_message = None;
        self.success_message = None;
        self.results.clear();
        
        let target = self.target_url.clone();
        let config = self.config.clone();
        let mode = self.scan_mode;
        let progress = Arc::clone(&self.progress);
        
        // Update progress status
        {
            let mut p = progress.lock();
            p.status = "Initializing scan...".to_string();
            p.current = 0;
            p.total = 0;
            p.discoveries = 0;
        }
        
        // Start scan in background thread
        let promise = Promise::spawn_thread("prune_scan", move || {
            // Create a new tokio runtime for this thread
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(scan_async(target, config, mode, progress))
        });
        
        self.scan_promise = Some(promise);
    }
    
    fn stop_scan(&mut self) {
        self.scanning = false;
        self.scan_promise = None;
        
        let mut p = self.progress.lock();
        p.status = "Scan stopped by user".to_string();
    }
    
    fn browse_wordlist(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text files", &["txt"])
            .add_filter("All files", &["*"])
            .pick_file()
        {
            self.custom_wordlist_path = Some(path.display().to_string());
            self.use_custom_wordlist = true;
        }
    }
}

impl eframe::App for PruneGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check if scan is complete
        if let Some(promise) = &self.scan_promise {
            if let Some(result) = promise.ready() {
                match result {
                    Ok(results) => {
                        self.results = results.clone();
                        self.success_message = Some(format!("Scan complete! Found {} results", results.len()));
                        self.scanning = false;
                    }
                    Err(e) => {
                        self.error_message = Some(e.clone());
                        self.scanning = false;
                    }
                }
                self.scan_promise = None;
            }
        }
        
        // Request repaint for animations
        if self.scanning {
            ctx.request_repaint();
        }
        
        // Top panel with title
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("🌿 Prune").size(32.0).color(egui::Color32::from_rgb(37, 150, 190)));
                ui.label(egui::RichText::new("Adaptive Discovery Engine").size(16.0).color(egui::Color32::from_rgb(86, 33, 213)));
            });
            ui.add_space(10.0);
        });
        
        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Target input section
                ui.group(|ui| {
                    ui.heading(egui::RichText::new("Target Configuration").color(egui::Color32::from_rgb(37, 150, 190)));
                    ui.add_space(5.0);
                    
                    ui.horizontal(|ui| {
                        ui.label("Target URL:");
                        ui.text_edit_singleline(&mut self.target_url)
                            .on_hover_text("Enter target URL (e.g., https://example.com) or domain (e.g., example.com)");
                    });
                    
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("Scan Mode:").strong());
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut self.scan_mode, ScanMode::Directory, "Directory Discovery");
                        ui.radio_value(&mut self.scan_mode, ScanMode::Subdomain, "Subdomain Enumeration");
                        ui.radio_value(&mut self.scan_mode, ScanMode::Both, "Combined (Both)");
                    });
                });
                
                ui.add_space(10.0);
                
                // Configuration section
                ui.group(|ui| {
                    ui.heading(egui::RichText::new("Scan Configuration").color(egui::Color32::from_rgb(37, 150, 190)));
                    ui.add_space(5.0);
                    
                    // Rate limiting
                    ui.horizontal(|ui| {
                        ui.label("Request Rate:");
                        ui.add(egui::Slider::new(&mut self.config.rate_limit, 10..=500).suffix(" req/s"));
                    });
                    
                    // Timeout
                    ui.horizontal(|ui| {
                        ui.label("Request Timeout:");
                        ui.add(egui::Slider::new(&mut self.config.timeout_seconds, 5..=60).suffix(" seconds"));
                    });
                    
                    // Crawling
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.config.crawling_enabled, "Enable passive crawling");
                    });
                    
                    // Max depth
                    if self.config.crawling_enabled {
                        ui.horizontal(|ui| {
                            ui.label("  Crawl depth:");
                            ui.add(egui::Slider::new(&mut self.config.max_depth, 1..=10));
                        });
                    }
                    
                    ui.add_space(10.0);
                    
                    // Excluded status codes
                    ui.label(egui::RichText::new("Excluded Status Codes:").strong());
                    ui.horizontal_wrapped(|ui| {
                        let codes = vec![404, 403, 429, 500, 502, 504, 405, 501];
                        for code in codes {
                            let mut included = self.config.excluded_status_codes.contains(&code);
                            if ui.checkbox(&mut included, format!("{}", code)).changed() {
                                if included {
                                    if !self.config.excluded_status_codes.contains(&code) {
                                        self.config.excluded_status_codes.push(code);
                                    }
                                } else {
                                    self.config.excluded_status_codes.retain(|&c| c != code);
                                }
                            }
                        }
                    });
                });
                
                ui.add_space(10.0);
                
                // Wordlist section
                ui.group(|ui| {
                    ui.heading(egui::RichText::new("Wordlist Options").color(egui::Color32::from_rgb(37, 150, 190)));
                    ui.add_space(5.0);
                    
                    ui.checkbox(&mut self.use_custom_wordlist, "Use custom wordlist");
                    
                    if self.use_custom_wordlist {
                        ui.horizontal(|ui| {
                            if let Some(ref path) = self.custom_wordlist_path {
                                ui.label(format!("Selected: {}", path));
                            } else {
                                ui.label("No wordlist selected");
                            }
                            
                            if ui.button("📁 Browse...").clicked() {
                                self.browse_wordlist();
                            }
                        });
                    } else {
                        ui.label("Using SecLists (if available) or built-in wordlists");
                    }
                });
                
                ui.add_space(10.0);
                
                // Action buttons
                ui.horizontal(|ui| {
                    if self.scanning {
                        if ui.button(egui::RichText::new("⏹ Stop Scan").size(18.0).color(egui::Color32::RED)).clicked() {
                            self.stop_scan();
                        }
                        
                        // Show spinner
                        ui.spinner();
                        
                        let progress = self.progress.lock();
                        ui.label(format!("Status: {}", progress.status));
                    } else {
                        if ui.button(egui::RichText::new("🚀 Start Scan").size(18.0).color(egui::Color32::from_rgb(37, 150, 190))).clicked() {
                            self.start_scan();
                        }
                    }
                });
                
                ui.add_space(10.0);
                
                // Messages
                if let Some(ref error) = self.error_message {
                    ui.colored_label(egui::Color32::RED, format!("❌ {}", error));
                }
                
                if let Some(ref success) = self.success_message {
                    ui.colored_label(egui::Color32::from_rgb(37, 150, 190), format!("✅ {}", success));
                }
                
                // Progress section
                if self.scanning {
                    ui.add_space(10.0);
                    ui.group(|ui| {
                        ui.heading("Scan Progress");
                        
                        let progress = self.progress.lock();
                        
                        if progress.total > 0 {
                            let progress_fraction = progress.current as f32 / progress.total as f32;
                            ui.add(egui::ProgressBar::new(progress_fraction)
                                .text(format!("{}/{} ({:.1}%)", progress.current, progress.total, progress_fraction * 100.0)));
                        } else {
                            ui.spinner();
                        }
                        
                        ui.horizontal(|ui| {
                            ui.label(format!("Rate: {:.1} req/s", progress.rate));
                            ui.separator();
                            ui.label(format!("Discoveries: {}", progress.discoveries));
                        });
                    });
                }
                
                // Results section
                if !self.results.is_empty() {
                    ui.add_space(10.0);
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.heading(format!("Results ({} found)", self.results.len()));
                            ui.separator();
                            let filter_response = ui.text_edit_singleline(&mut self.filter_text);
                            if self.filter_text.is_empty() {
                                filter_response.on_hover_text("Filter results...");
                            }
                            ui.checkbox(&mut self.show_only_200, "Show only 200 OK");
                        });
                        
                        ui.separator();
                        
                        egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
                            egui::Grid::new("results_grid")
                                .striped(true)
                                .spacing([10.0, 4.0])
                                .show(ui, |ui| {
                                    // Header
                                    ui.label(egui::RichText::new("Status").strong());
                                    ui.label(egui::RichText::new("URL").strong());
                                    ui.label(egui::RichText::new("Size").strong());
                                    ui.label(egui::RichText::new("Confidence").strong());
                                    ui.end_row();
                                    
                                    // Filter and display results
                                    for result in &self.results {
                                        // Apply filters
                                        if self.show_only_200 && result.status_code != 200 {
                                            continue;
                                        }
                                        
                                        if !self.filter_text.is_empty() && !result.url.contains(&self.filter_text) {
                                            continue;
                                        }
                                        
                                        // Status code with color
                                        let status_color = match result.status_code {
                                            200..=299 => egui::Color32::from_rgb(37, 150, 190),
                                            300..=399 => egui::Color32::from_rgb(86, 33, 213),
                                            400..=499 => egui::Color32::YELLOW,
                                            _ => egui::Color32::RED,
                                        };
                                        ui.colored_label(status_color, result.status_code.to_string());
                                        
                                        // URL (clickable)
                                        ui.hyperlink_to(&result.url, &result.url);
                                        
                                        // Size
                                        ui.label(format_size(result.size));
                                        
                                        // Confidence indicator
                                        let confidence_text = if result.confidence > 0.8 {
                                            "●●●"
                                        } else if result.confidence > 0.5 {
                                            "●●○"
                                        } else {
                                            "●○○"
                                        };
                                        ui.label(confidence_text);
                                        
                                        ui.end_row();
                                    }
                                });
                        });
                        
                        ui.separator();
                        
                        // Export button
                        if ui.button("💾 Export Results").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("JSON", &["json"])
                                .add_filter("CSV", &["csv"])
                                .save_file()
                            {
                                if let Err(e) = export_results(&self.results, &path) {
                                    self.error_message = Some(format!("Failed to export: {}", e));
                                } else {
                                    self.success_message = Some(format!("Exported to {}", path.display()));
                                }
                            }
                        }
                    });
                }
            });
        });
    }
}

/// Run the scan asynchronously
async fn scan_async(
    target: String,
    config: ScanConfig,
    mode: ScanMode,
    progress: Arc<Mutex<ScanProgress>>,
) -> Result<Vec<DiscoveryResult>, String> {
    // Update progress
    {
        let mut p = progress.lock();
        p.status = "Creating discovery engine...".to_string();
    }
    
    // Create engine
    let mut engine = DiscoveryEngine::new(target.clone(), config)
        .await
        .map_err(|e| e.to_string())?;
    
    engine.set_mode(mode);
    
    // Update progress
    {
        let mut p = progress.lock();
        p.status = format!("Starting {} scan...", match mode {
            ScanMode::Directory => "directory",
            ScanMode::Subdomain => "subdomain",
            ScanMode::Both => "combined",
        });
    }
    
    // Run scan based on mode
    match mode {
        ScanMode::Directory => {
            engine.run_directory_discovery()
                .await
                .map_err(|e| e.to_string())?;
        }
        ScanMode::Subdomain => {
            engine.run_subdomain_discovery()
                .await
                .map_err(|e| e.to_string())?;
        }
        ScanMode::Both => {
            engine.run_combined_discovery()
                .await
                .map_err(|e| e.to_string())?;
        }
    }
    
    let results = engine.get_results();
    
    // Update final progress
    {
        let mut p = progress.lock();
        p.status = "Scan complete!".to_string();
        p.discoveries = results.len();
    }
    
    Ok(results)
}

/// Format file size
fn format_size(bytes: usize) -> String {
    if bytes == 0 {
        return "-".to_string();
    }
    
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_idx])
}

/// Export results to file
fn export_results(results: &[DiscoveryResult], path: &std::path::Path) -> anyhow::Result<()> {
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    
    match extension {
        "json" => {
            let json = serde_json::to_string_pretty(results)?;
            std::fs::write(path, json)?;
        }
        "csv" => {
            let mut csv = String::from("Status,URL,Size,Confidence,Discovered At\n");
            for result in results {
                csv.push_str(&format!(
                    "{},{},{},{},{}\n",
                    result.status_code,
                    result.url,
                    result.size,
                    result.confidence,
                    result.discovered_at
                ));
            }
            std::fs::write(path, csv)?;
        }
        _ => {
            anyhow::bail!("Unsupported file format: {}", extension);
        }
    }
    
    Ok(())
}

/// Launch the GUI application
pub fn launch_gui() -> anyhow::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 900.0])
            .with_min_inner_size([800.0, 600.0])
            .with_icon(create_icon()),
        ..Default::default()
    };
    
    eframe::run_native(
        "Prune - Adaptive Discovery Engine",
        options,
        Box::new(|cc| Ok(Box::new(PruneGui::new(cc)))),
    ).map_err(|e| anyhow::anyhow!("Failed to launch GUI: {}", e))?;
    
    Ok(())
}

/// Create application icon
fn create_icon() -> egui::IconData {
    // Simple icon data (16x16 blue square with leaf pattern)
    let icon_size = 16;
    let mut rgba = vec![0u8; icon_size * icon_size * 4];
    
    for y in 0..icon_size {
        for x in 0..icon_size {
            let idx = (y * icon_size + x) * 4;
            // Blue color #2596be
            rgba[idx] = 37;      // R
            rgba[idx + 1] = 150; // G
            rgba[idx + 2] = 190; // B
            rgba[idx + 3] = 255; // A
        }
    }
    
    egui::IconData {
        rgba,
        width: icon_size as u32,
        height: icon_size as u32,
    }
}
