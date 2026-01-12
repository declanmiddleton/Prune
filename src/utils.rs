/// Utility functions and helpers

/// Validate URL format
pub fn is_valid_url(url: &str) -> bool {
    url::Url::parse(url).is_ok()
        || url::Url::parse(&format!("http://{}", url)).is_ok()
}

/// Validate domain format
pub fn is_valid_domain(domain: &str) -> bool {
    // Basic domain validation
    let parts: Vec<&str> = domain.split('.').collect();
    parts.len() >= 2 && parts.iter().all(|p| !p.is_empty())
}

/// Extract file extension from path
pub fn get_file_extension(path: &str) -> Option<String> {
    path.rsplit('.')
        .next()
        .filter(|ext| ext.len() <= 10 && !ext.contains('/'))
        .map(|s| s.to_lowercase())
}

/// Check if path looks like a file (has extension)
pub fn is_likely_file(path: &str) -> bool {
    get_file_extension(path).is_some()
}

/// Sanitize path for safe filesystem operations
pub fn sanitize_path(path: &str) -> String {
    path.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}

/// Generate a unique identifier
pub fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("{:x}", timestamp)
}

/// Calculate similarity between two strings (simple Levenshtein-like)
pub fn string_similarity(a: &str, b: &str) -> f32 {
    if a == b {
        return 1.0;
    }
    
    let len_a = a.len();
    let len_b = b.len();
    
    if len_a == 0 || len_b == 0 {
        return 0.0;
    }
    
    let max_len = len_a.max(len_b);
    let common = a.chars()
        .filter(|c| b.contains(*c))
        .count();
    
    common as f32 / max_len as f32
}

/// Parse status code color category
pub fn status_category(code: u16) -> &'static str {
    match code {
        200..=299 => "success",
        300..=399 => "redirect",
        400..=499 => "client_error",
        500..=599 => "server_error",
        _ => "unknown",
    }
}

/// Check if status code is informative
pub fn is_informative_status(code: u16) -> bool {
    matches!(code, 200..=299 | 301 | 302 | 401 | 403)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_url() {
        assert!(is_valid_url("https://example.com"));
        assert!(is_valid_url("http://example.com"));
        assert!(is_valid_url("example.com"));
        assert!(!is_valid_url("not a url"));
    }
    
    #[test]
    fn test_file_extension() {
        assert_eq!(get_file_extension("test.txt"), Some("txt".to_string()));
        assert_eq!(get_file_extension("file.tar.gz"), Some("gz".to_string()));
        assert_eq!(get_file_extension("noextension"), None);
    }
    
    #[test]
    fn test_string_similarity() {
        assert_eq!(string_similarity("test", "test"), 1.0);
        assert!(string_similarity("test", "text") > 0.5);
        assert!(string_similarity("abc", "xyz") < 0.5);
    }
}
