use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub session_id: String,
    pub target: String,
    pub mode: ScanMode,
    pub started_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub results: Vec<DiscoveryResult>,
    pub config: ScanConfig,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ScanMode {
    Directory,
    Subdomain,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryResult {
    pub url: String,
    pub status_code: u16,
    pub size: usize,
    pub confidence: f32,
    pub discovered_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    pub crawling_enabled: bool,
    pub rate_limit: u64,
    pub excluded_status_codes: Vec<u16>,
    pub max_depth: usize,
    pub timeout_seconds: u64,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            crawling_enabled: false,
            rate_limit: 100,
            excluded_status_codes: vec![404, 405, 501],
            max_depth: 3,
            timeout_seconds: 10,
        }
    }
}

pub struct SessionManager {
    sessions_dir: PathBuf,
    config_file: PathBuf,
}

impl SessionManager {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot find home directory"))?;
        let prune_dir = home.join(".prune");
        let sessions_dir = prune_dir.join("sessions");
        let config_file = prune_dir.join("config.json");
        
        // Create directories if they don't exist
        fs::create_dir_all(&sessions_dir)?;
        
        Ok(Self {
            sessions_dir,
            config_file,
        })
    }
    
    /// Create a new session
    pub fn create_session(&self, target: &str, mode: ScanMode) -> Result<String> {
        let session_id = format!("prune_{}", Utc::now().timestamp());
        let session_data = SessionData {
            session_id: session_id.clone(),
            target: target.to_string(),
            mode,
            started_at: Utc::now(),
            updated_at: Utc::now(),
            results: Vec::new(),
            config: self.load_config()?,
        };
        
        self.save_session(&session_id, &session_data.results)?;
        
        Ok(session_id)
    }
    
    /// Save session results
    pub fn save_session(&self, session_id: &str, results: &[DiscoveryResult]) -> Result<()> {
        let session_file = self.sessions_dir.join(format!("{}.json", session_id));
        
        // Load existing session data if it exists, otherwise create new
        let mut session_data = if session_file.exists() {
            let content = fs::read_to_string(&session_file)?;
            serde_json::from_str::<SessionData>(&content)?
        } else {
            // Create minimal session data
            SessionData {
                session_id: session_id.to_string(),
                target: String::new(),
                mode: ScanMode::Directory,
                started_at: Utc::now(),
                updated_at: Utc::now(),
                results: Vec::new(),
                config: self.load_config()?,
            }
        };
        
        session_data.results = results.to_vec();
        session_data.updated_at = Utc::now();
        
        let json = serde_json::to_string_pretty(&session_data)?;
        fs::write(session_file, json)?;
        
        Ok(())
    }
    
    /// Get the last session
    pub fn get_last_session(&self) -> Result<Option<(String, SessionData)>> {
        let mut sessions: Vec<_> = fs::read_dir(&self.sessions_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext == "json")
                    .unwrap_or(false)
            })
            .collect();
        
        if sessions.is_empty() {
            return Ok(None);
        }
        
        sessions.sort_by_key(|entry| {
            entry.metadata()
                .and_then(|m| m.modified())
                .ok()
        });
        
        if let Some(last) = sessions.last() {
            let content = fs::read_to_string(last.path())?;
            let session_data: SessionData = serde_json::from_str(&content)?;
            let session_id = last.path()
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            return Ok(Some((session_id, session_data)));
        }
        
        Ok(None)
    }
    
    /// Load configuration
    pub fn load_config(&self) -> Result<ScanConfig> {
        if self.config_file.exists() {
            let content = fs::read_to_string(&self.config_file)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(ScanConfig::default())
        }
    }
    
    /// Save configuration
    pub fn save_config(&self, config: &ScanConfig) -> Result<()> {
        let json = serde_json::to_string_pretty(config)?;
        fs::write(&self.config_file, json)?;
        Ok(())
    }
}
