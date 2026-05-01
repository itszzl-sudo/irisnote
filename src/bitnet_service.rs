use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BitNetConfig {
    pub enabled: bool,
    pub max_keywords: usize,
}

impl Default for BitNetConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_keywords: 5,
        }
    }
}

pub struct BitNetService {
    config: std::sync::RwLock<BitNetConfig>,
}

impl BitNetService {
    pub fn new() -> Self {
        Self {
            config: std::sync::RwLock::new(BitNetConfig::default()),
        }
    }
    
    pub fn with_config(config: BitNetConfig) -> Self {
        Self {
            config: std::sync::RwLock::new(config),
        }
    }
    
    pub fn summarize_content(&self, content: &str) -> Result<String, Box<dyn std::error::Error>> {
        let config = self.config.read().unwrap();
        
        if !config.enabled {
            return Err("BitNet service is disabled".into());
        }
        
        let keywords = self.extract_keywords(content, config.max_keywords);
        
        if keywords.is_empty() {
            return Ok("untitled".to_string());
        }
        
        Ok(keywords.join("_"))
    }
    
    fn extract_keywords(&self, content: &str, max_keywords: usize) -> Vec<String> {
        let mut word_freq: HashMap<String, usize> = HashMap::new();
        
        let stop_words = [
            "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for",
            "of", "with", "by", "from", "is", "are", "was", "were", "be", "been",
            "being", "have", "has", "had", "do", "does", "did", "will", "would",
            "could", "should", "may", "might", "must", "can", "this", "that",
            "these", "those", "it", "its", "as", "if", "then", "else", "when",
            "的", "了", "和", "是", "在", "有", "我", "他", "她", "它", "们",
            "这", "那", "也", "就", "都", "而", "及", "与", "或", "但", "如",
            "fn", "let", "mut", "if", "else", "for", "while", "return", "use",
            "mod", "pub", "struct", "enum", "impl", "trait", "type", "const",
            "def", "class", "import", "from", "as", "try", "except", "raise",
            "function", "var", "const", "let", "new", "this", "super",
        ];
        
        let words: Vec<String> = content
            .to_lowercase()
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .filter(|w| w.len() > 2)
            .filter(|w| !stop_words.contains(&&w[..]))
            .map(|w| w.to_string())
            .collect();
        
        for word in words {
            *word_freq.entry(word).or_insert(0) += 1;
        }
        
        let mut freq_vec: Vec<(String, usize)> = word_freq.into_iter().collect();
        freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
        
        freq_vec
            .into_iter()
            .take(max_keywords)
            .map(|(word, _)| word)
            .collect()
    }
}
