use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub recent_paths: Vec<PathBuf>,
    pub default_save_dir: Option<PathBuf>,
    pub auto_detect_type: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            recent_paths: Vec::new(),
            default_save_dir: dirs::document_dir(),
            auto_detect_type: true,
        }
    }
}

impl Config {
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("smart-text-editor").join("config.json"))
    }
    
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::config_path().ok_or("无法确定配置路径")?;
        
        if !path.exists() {
            return Ok(Self::default());
        }
        
        let content = std::fs::read_to_string(&path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::config_path().ok_or("无法确定配置路径")?;
        
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }
}
