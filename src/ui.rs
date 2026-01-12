use colored::Colorize;
use std::io::{self, Write};

// Color scheme matching CAP tool
pub const PRIMARY_COLOR: &str = "#2596be";
pub const SECONDARY_COLOR: &str = "#5621d5";
pub const SUCCESS_COLOR: &str = "#2596be";
pub const WARNING_COLOR: &str = "#ffa500";
pub const ERROR_COLOR: &str = "#ff6b6b";

/// Display the Prune banner
pub fn display_banner() {
    let banner = r#"
 ____  ____  _  _  __ _  ____ 
(  _ \(  _ \/ )( \(  ( \(  __)
 ) __/ )   /) \/ (/    / ) _) 
(__)  (__\_)\____/\_)__)(____)
"#;
    
    println!("{}", banner.truecolor(37, 150, 190).bold());
    println!("{}", "Adaptive Discovery Engine".truecolor(86, 33, 213));
    println!("{}", "━".repeat(60).truecolor(86, 33, 213));
    println!();
}

/// Print a section header
pub fn print_section_header(title: &str) {
    println!("\n{}", "━".repeat(60).truecolor(86, 33, 213));
    println!("{}", title.truecolor(37, 150, 190).bold());
    println!("{}", "━".repeat(60).truecolor(86, 33, 213));
}

/// Print informational message
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".truecolor(86, 33, 213), message);
}

/// Print success message
pub fn print_success(message: &str) {
    println!("{} {}", "✓".truecolor(37, 150, 190).bold(), message.truecolor(37, 150, 190));
}

/// Print warning message
pub fn print_warning(message: &str) {
    println!("{} {}", "⚠".truecolor(255, 165, 0), message.truecolor(255, 165, 0));
}

/// Print error message
pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".truecolor(255, 107, 107), message.truecolor(255, 107, 107));
}

/// Print discovery finding
pub fn print_finding(status: u16, url: &str, size: usize, confidence: f32) {
    let status_color = match status {
        200..=299 => (37, 150, 190),
        300..=399 => (86, 33, 213),
        400..=499 => (255, 165, 0),
        _ => (255, 107, 107),
    };
    
    let confidence_indicator = if confidence > 0.8 {
        "●●●".truecolor(37, 150, 190)
    } else if confidence > 0.5 {
        "●●○".truecolor(86, 33, 213)
    } else {
        "●○○".truecolor(100, 100, 100)
    };
    
    println!(
        "{} {} {} {} {}",
        status.to_string().truecolor(status_color.0, status_color.1, status_color.2).bold(),
        "│".truecolor(86, 33, 213),
        url.truecolor(37, 150, 190),
        format_size(size).truecolor(86, 33, 213),
        confidence_indicator
    );
}

/// Print learning adaptation message
pub fn print_adaptation(message: &str) {
    println!("{} {}", "⚙".truecolor(86, 33, 213), message.truecolor(86, 33, 213).italic());
}

/// Print progress with animated indicator
pub fn print_progress(current: usize, total: usize, rate: f32, discoveries: usize) {
    let percentage = (current as f32 / total as f32 * 100.0) as usize;
    let bar_width = 40;
    let filled = (bar_width as f32 * (current as f32 / total as f32)) as usize;
    
    let bar: String = (0..bar_width)
        .map(|i| if i < filled { '█' } else { '░' })
        .collect();
    
    print!("\r{} {} {}% {} {} req/s {} {} discoveries",
        "→".truecolor(37, 150, 190).bold(),
        bar.truecolor(37, 150, 190),
        percentage.to_string().truecolor(86, 33, 213).bold(),
        "│".truecolor(86, 33, 213),
        format!("{:.1}", rate).truecolor(86, 33, 213),
        "│".truecolor(86, 33, 213),
        discoveries.to_string().truecolor(37, 150, 190).bold()
    );
    
    io::stdout().flush().ok();
}

/// Clear current line
pub fn clear_line() {
    print!("\r{}\r", " ".repeat(100));
    io::stdout().flush().ok();
}

/// Print intelligence summary
pub fn print_intelligence_summary(
    excluded_codes: &[u16],
    wildcards: usize,
    mutations: usize,
    confidence: f32,
) {
    println!("\n{}", "Intelligence Summary".truecolor(86, 33, 213).bold());
    println!("{}", "─".repeat(60).truecolor(86, 33, 213));
    
    if !excluded_codes.is_empty() {
        println!(
            "  {} {}",
            "Excluded codes:".truecolor(86, 33, 213),
            format!("{:?}", excluded_codes).truecolor(37, 150, 190)
        );
    }
    
    if wildcards > 0 {
        println!(
            "  {} {}",
            "Wildcard patterns:".truecolor(86, 33, 213),
            wildcards.to_string().truecolor(37, 150, 190)
        );
    }
    
    if mutations > 0 {
        println!(
            "  {} {}",
            "Generated mutations:".truecolor(86, 33, 213),
            mutations.to_string().truecolor(37, 150, 190)
        );
    }
    
    println!(
        "  {} {}",
        "Overall confidence:".truecolor(86, 33, 213),
        format!("{:.1}%", confidence * 100.0).truecolor(37, 150, 190)
    );
}

/// Animate a pulse effect for active operations
pub fn pulse_indicator(text: &str) {
    print!("\r{} {}", "◉".truecolor(37, 150, 190), text.truecolor(86, 33, 213));
    io::stdout().flush().ok();
}

/// Format size helper
fn format_size(bytes: usize) -> String {
    if bytes == 0 {
        return "".to_string();
    }
    
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    
    format!("({:.1}{})", size, UNITS[unit_idx])
}
