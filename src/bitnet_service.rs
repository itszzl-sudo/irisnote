use std::sync::Arc;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitNetConfig {
    pub model_path: Option<PathBuf>,
    pub enabled: bool,
    pub max_tokens: usize,
    pub temperature: f64,
}

impl Default for BitNetConfig {
    fn default() -> Self {
        Self {
            model_path: None,
            enabled: true,
            max_tokens: 20,
            temperature: 0.7,
        }
    }
}

pub struct BitNetService {
    config: Arc<std::sync::RwLock<BitNetConfig>>,
    model_loaded: bool,
}

impl BitNetService {
    pub fn new() -> Self {
        Self {
            config: Arc::new(std::sync::RwLock::new(BitNetConfig::default())),
            model_loaded: false,
        }
    }
    
    pub fn with_config(config: BitNetConfig) -> Self {
        Self {
            config: Arc::new(std::sync::RwLock::new(config)),
            model_loaded: false,
        }
    }
    
    pub fn summarize_content(&self, content: &str) -> Result<String, Box<dyn std::error::Error>> {
        let config = self.config.read().unwrap();
        
        if !config.enabled {
            return Err("BitNet service is disabled".into());
        }
        
        let content_preview = content.chars().take(300).collect::<String>();
        
        let title = self.extract_keywords(&content_preview);
        
        if title.is_empty() {
            Ok("untitled".to_string())
        } else {
            Ok(title)
        }
    }
    
    fn extract_keywords(&self, content: &str) -> String {
        let words = content
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .filter(|w| w.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-'))
            .take(50)
            .collect::<Vec<_>>();
        
        if words.is_empty() {
            return String::new();
        }
        
        let mut word_freq = std::collections::HashMap::new();
        for word in &words {
            *word_freq.entry(word.to_lowercase()).or_insert(0) += 1;
        }
        
        let mut freq_vec: Vec<_> = word_freq.into_iter().collect();
        freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
        
        let stop_words = vec![
            "this", "that", "with", "from", "have", "been", "will", "would",
            "could", "should", "their", "there", "which", "when", "what",
            "about", "into", "more", "some", "than", "other", "only", "also",
            "just", "like", "such", "make", "made", "time", "very", "after",
            "before", "being", "over", "both", "through", "during", "between",
            "under", "again", "further", "then", "once", "here", "where",
            "while", "each", "few", "because", "most", "even", "does", "doing",
            "them", "they", "these", "those", "first", "second", "third",
            "using", "used", "uses", "following", "followed", "example",
            "function", "class", "method", "variable", "value", "result",
            "data", "type", "object", "string", "number", "boolean",
        ];
        
        let keywords: Vec<String> = freq_vec
            .iter()
            .filter(|(word, _)| !stop_words.contains(&word.as_str()))
            .take(3)
            .map(|(word, _)| word.clone())
            .collect();
        
        keywords.join(" ")
    }
    
    pub fn is_available(&self) -> bool {
        let config = self.config.read().unwrap();
        config.enabled
    }
    
    pub fn set_enabled(&self, enabled: bool) {
        let mut config = self.config.write().unwrap();
        config.enabled = enabled;
    }
    
    pub fn get_config(&self) -> BitNetConfig {
        self.config.read().unwrap().clone()
    }
}

impl Default for BitNetService {
    fn default() -> Self {
        Self::new()
    }
}
