use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub model_path: String,
    pub server_url: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub enabled: bool,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            model_path: r"C:\Users\a\Downloads\Qwen2.5-Coder-0.5B-Instruct-Q4_K_M.gguf".to_string(),
            server_url: "http://localhost:8080".to_string(),
            max_tokens: 50,
            temperature: 0.7,
            enabled: true,
        }
    }
}

pub struct LLMService {
    config: Arc<RwLock<LLMConfig>>,
    client: reqwest::Client,
    server_process: Option<Arc<std::process::Child>>,
}

impl LLMService {
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(LLMConfig::default())),
            client: reqwest::Client::new(),
            server_process: None,
        }
    }
    
    pub fn with_config(config: LLMConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            client: reqwest::Client::new(),
            server_process: None,
        }
    }
    
    pub async fn summarize_content(&self, content: &str) -> Result<String, Box<dyn std::error::Error>> {
        let config = self.config.read().await;
        
        if !config.enabled {
            return Err("LLM service is disabled".into());
        }
        
        let prompt = format!(
            "Please provide a concise title (2-5 words) that summarizes the main topic of this content:\n\n{}\n\nTitle:",
            content.chars().take(500).collect::<String>()
        );
        
        let request = LLMRequest {
            prompt,
            max_tokens: config.max_tokens,
            temperature: config.temperature,
        };
        
        let url = format!("{}/completion", config.server_url);
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if response.status().is_success() {
            let result: LLMResponse = response.json().await?;
            Ok(self.clean_title(&result.content))
        } else {
            Err(format!("LLM request failed: {}", response.status()).into())
        }
    }
    
    fn clean_title(&self, title: &str) -> String {
        let cleaned = title
            .trim()
            .replace("\"", "")
            .replace("'", "")
            .replace("\n", " ")
            .split_whitespace()
            .take(5)
            .collect::<Vec<_>>()
            .join(" ");
        
        if cleaned.is_empty() {
            "untitled".to_string()
        } else {
            cleaned.to_lowercase()
        }
    }
    
    pub async fn is_available(&self) -> bool {
        let config = self.config.read().await;
        
        if !config.enabled {
            return false;
        }
        
        let url = format!("{}/health", config.server_url);
        
        match self.client.get(&url).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
    
    pub async fn set_enabled(&self, enabled: bool) {
        let mut config = self.config.write().await;
        config.enabled = enabled;
    }
    
    pub async fn update_config(&self, new_config: LLMConfig) {
        let mut config = self.config.write().await;
        *config = new_config;
    }
    
    pub async fn get_config(&self) -> LLMConfig {
        self.config.read().await.clone()
    }
    
    pub async fn check_available(&self) -> Option<bool> {
        Some(self.is_available().await)
    }
}

#[derive(Debug, Serialize)]
struct LLMRequest {
    prompt: String,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
struct LLMResponse {
    content: String,
}

impl Drop for LLMService {
    fn drop(&mut self) {
        if let Some(process) = &self.server_process {
            let mut p = Arc::try_unwrap(process.clone()).unwrap();
            let _ = p.kill();
        }
    }
}
