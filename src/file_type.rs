use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::llm_service::{LLMService, LLMConfig};

#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    PlainText,
    Markdown,
    Rust,
    Python,
    JavaScript,
    TypeScript,
    HTML,
    CSS,
    JSON,
    XML,
    YAML,
    TOML,
    SVG,
    Image(String),
    C,
    CPP,
    Java,
    Go,
}

pub fn get_supported_extensions() -> Vec<String> {
    vec![
        "txt".to_string(),
        "md".to_string(),
        "rs".to_string(),
        "py".to_string(),
        "js".to_string(),
        "ts".to_string(),
        "html".to_string(),
        "htm".to_string(),
        "css".to_string(),
        "json".to_string(),
        "xml".to_string(),
        "yaml".to_string(),
        "yml".to_string(),
        "toml".to_string(),
        "svg".to_string(),
        "c".to_string(),
        "cpp".to_string(),
        "java".to_string(),
        "go".to_string(),
    ]
}

pub fn detect_file_type(content: &str, path: Option<&Path>) -> FileType {
    if let Some(p) = path {
        if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
            return match ext.to_lowercase().as_str() {
                "md" => FileType::Markdown,
                "rs" => FileType::Rust,
                "py" => FileType::Python,
                "js" => FileType::JavaScript,
                "ts" => FileType::TypeScript,
                "html" | "htm" => FileType::HTML,
                "css" => FileType::CSS,
                "json" => FileType::JSON,
                "xml" => FileType::XML,
                "yaml" | "yml" => FileType::YAML,
                "toml" => FileType::TOML,
                "svg" => FileType::SVG,
                "png" | "jpg" | "jpeg" | "gif" | "bmp" => FileType::Image(ext.to_string()),
                "c" => FileType::C,
                "cpp" | "cc" | "cxx" => FileType::CPP,
                "java" => FileType::Java,
                "go" => FileType::Go,
                _ => detect_from_content(content),
            };
        }
    }
    
    detect_from_content(content)
}

fn detect_from_content(content: &str) -> FileType {
    let trimmed = content.trim();
    
    if trimmed.is_empty() {
        return FileType::PlainText;
    }
    
    if trimmed.starts_with("<?xml") || trimmed.starts_with("<") && trimmed.contains("xmlns") {
        return FileType::XML;
    }
    
    if trimmed.starts_with("<svg") || trimmed.contains("xmlns=\"http://www.w3.org/2000/svg\"") {
        return FileType::SVG;
    }
    
    if trimmed.starts_with("<!DOCTYPE html") || trimmed.starts_with("<html") {
        return FileType::HTML;
    }
    
    if trimmed.starts_with("---\n") && content.contains("\n---\n") {
        return FileType::YAML;
    }
    
    if (trimmed.starts_with('{') && trimmed.ends_with('}')) || 
       (trimmed.starts_with('[') && trimmed.ends_with(']')) {
        if serde_json::from_str::<serde_json::Value>(trimmed).is_ok() {
            return FileType::JSON;
        }
    }
    
    if trimmed.starts_with("fn ") || trimmed.contains("fn main()") || 
       trimmed.contains("let mut") || trimmed.contains("impl ") {
        return FileType::Rust;
    }
    
    if trimmed.starts_with("def ") || trimmed.starts_with("import ") || 
       trimmed.starts_with("from ") || trimmed.contains("if __name__") {
        return FileType::Python;
    }
    
    if trimmed.starts_with("package ") || trimmed.contains("func main()") {
        return FileType::Go;
    }
    
    if trimmed.starts_with("#include") || trimmed.contains("int main(") {
        if trimmed.contains("std::") || trimmed.contains("class ") {
            return FileType::CPP;
        }
        return FileType::C;
    }
    
    if trimmed.starts_with("public class") || trimmed.starts_with("class ") && trimmed.contains("public static void main") {
        return FileType::Java;
    }
    
    if trimmed.starts_with("function ") || trimmed.starts_with("const ") || 
       trimmed.starts_with("let ") || trimmed.contains("=>") {
        if trimmed.contains(": ") || trimmed.contains("interface ") {
            return FileType::TypeScript;
        }
        return FileType::JavaScript;
    }
    
    if trimmed.starts_with(".") || trimmed.starts_with("#") && trimmed.contains("{") {
        return FileType::CSS;
    }
    
    if trimmed.contains("# ") || trimmed.contains("## ") || 
       trimmed.contains("* ") || trimmed.contains("```") {
        return FileType::Markdown;
    }
    
    if trimmed.starts_with("[") && trimmed.contains("=") {
        return FileType::TOML;
    }
    
    FileType::PlainText
}

pub struct FilenameSuggester {
    llm_service: Arc<RwLock<Option<LLMService>>>,
}

impl FilenameSuggester {
    pub fn new() -> Self {
        Self {
            llm_service: Arc::new(RwLock::new(None)),
        }
    }
    
    pub fn with_llm(llm_config: LLMConfig) -> Self {
        Self {
            llm_service: Arc::new(RwLock::new(Some(LLMService::with_config(llm_config)))),
        }
    }
    
    pub async fn set_llm_service(&self, config: LLMConfig) {
        let mut service = self.llm_service.write().await;
        *service = Some(LLMService::with_config(config));
    }
    
    pub async fn suggest_filename(
        &self,
        content: &str,
        path: Option<&Path>,
        file_type: &FileType,
    ) -> String {
        if let Some(p) = path {
            if let Some(filename) = p.file_name().and_then(|f| f.to_str()) {
                return filename.to_string();
            }
        }
        
        let ext = get_extension_for_type(file_type);
        
        let suggested_name = self.extract_meaningful_name(content, file_type).await;
        
        format!("{}.{}", suggested_name, ext)
    }
    
    async fn extract_meaningful_name(&self, content: &str, file_type: &FileType) -> String {
        let lines: Vec<&str> = content.lines().take(20).collect();
        
        let name = match file_type {
            FileType::Rust => self.extract_rust_name(&lines),
            FileType::Python => self.extract_python_name(&lines),
            FileType::JavaScript | FileType::TypeScript => self.extract_js_name(&lines),
            FileType::Markdown => self.extract_markdown_name(&lines),
            FileType::HTML => self.extract_html_name(content),
            FileType::Go => self.extract_go_name(&lines),
            FileType::Java => self.extract_java_name(&lines),
            FileType::C | FileType::CPP => self.extract_c_name(&lines),
            FileType::JSON => self.extract_json_name(content),
            FileType::TOML => self.extract_toml_name(&lines),
            _ => None,
        };
        
        if let Some(n) = name {
            return n;
        }
        
        if content.len() > 50 {
            if let Some(llm) = self.llm_service.read().await.as_ref() {
                if llm.is_available().await {
                    match llm.summarize_content(content).await {
                        Ok(title) => {
                            if !title.is_empty() && title != "untitled" {
                                return title;
                            }
                        }
                        Err(e) => {
                            eprintln!("LLM 总结失败: {}", e);
                        }
                    }
                }
            }
        }
        
        "untitled".to_string()
    }
    
    fn extract_rust_name(&self, lines: &[&str]) -> Option<String> {
        for line in lines {
            if line.starts_with("fn ") {
                if let Some(name) = line.strip_prefix("fn ") {
                    let name = name.split('(').next().unwrap_or("main");
                    return Some(sanitize_name(name));
                }
            }
            if line.starts_with("mod ") {
                if let Some(name) = line.strip_prefix("mod ") {
                    let name = name.trim().trim_end_matches(';');
                    return Some(sanitize_name(name));
                }
            }
        }
        None
    }
    
    fn extract_python_name(&self, lines: &[&str]) -> Option<String> {
        for line in lines {
            if line.starts_with("def ") {
                if let Some(name) = line.strip_prefix("def ") {
                    let name = name.split('(').next().unwrap_or("main");
                    return Some(sanitize_name(name));
                }
            }
            if line.starts_with("class ") {
                if let Some(name) = line.strip_prefix("class ") {
                    let name = name.split(':').next().unwrap_or("MyClass");
                    return Some(sanitize_name(name));
                }
            }
        }
        None
    }
    
    fn extract_js_name(&self, lines: &[&str]) -> Option<String> {
        for line in lines {
            if line.starts_with("function ") {
                if let Some(name) = line.strip_prefix("function ") {
                    let name = name.split('(').next().unwrap_or("main");
                    return Some(sanitize_name(name));
                }
            }
            if line.contains("const ") && line.contains(" = ") {
                for part in line.split("const ") {
                    if part.contains(" = ") {
                        let name = part.split(" = ").next().unwrap_or("main");
                        return Some(sanitize_name(name.trim()));
                    }
                }
            }
        }
        None
    }
    
    fn extract_markdown_name(&self, lines: &[&str]) -> Option<String> {
        for line in lines {
            if line.starts_with("# ") {
                if let Some(title) = line.strip_prefix("# ") {
                    return Some(sanitize_name(title));
                }
            }
        }
        None
    }
    
    fn extract_html_name(&self, content: &str) -> Option<String> {
        if content.contains("<title>") {
            if let Some(start) = content.find("<title>") {
                if let Some(end) = content.find("</title>") {
                    if end > start {
                        let title = &content[start + 7..end];
                        return Some(sanitize_name(title));
                    }
                }
            }
        }
        None
    }
    
    fn extract_go_name(&self, lines: &[&str]) -> Option<String> {
        for line in lines {
            if line.starts_with("func ") {
                if let Some(name) = line.strip_prefix("func ") {
                    let name = name.split('(').next().unwrap_or("main");
                    return Some(sanitize_name(name));
                }
            }
        }
        None
    }
    
    fn extract_java_name(&self, lines: &[&str]) -> Option<String> {
        for line in lines {
            if line.contains("class ") {
                for part in line.split("class ") {
                    if !part.is_empty() {
                        let name = part.split_whitespace().next().unwrap_or("Main");
                        return Some(sanitize_name(name));
                    }
                }
            }
        }
        None
    }
    
    fn extract_c_name(&self, lines: &[&str]) -> Option<String> {
        for line in lines {
            if line.starts_with("int ") && line.contains("main") {
                return Some("main".to_string());
            }
        }
        None
    }
    
    fn extract_json_name(&self, content: &str) -> Option<String> {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(content) {
            if let Some(obj) = json.as_object() {
                if obj.contains_key("name") {
                    if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
                        return Some(sanitize_name(name));
                    }
                }
                if obj.contains_key("package") {
                    if let Some(name) = obj.get("package").and_then(|v| v.as_str()) {
                        return Some(sanitize_name(name));
                    }
                }
            }
        }
        None
    }
    
    fn extract_toml_name(&self, lines: &[&str]) -> Option<String> {
        for line in lines {
            if line.starts_with("name = ") {
                if let Some(name) = line.strip_prefix("name = ") {
                    return Some(sanitize_name(name.trim_matches('"')));
                }
            }
        }
        None
    }
}

fn sanitize_name(name: &str) -> String {
    let name = name.trim();
    let sanitized: String = name
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect();
    
    if sanitized.is_empty() {
        "untitled".to_string()
    } else {
        sanitized.to_lowercase()
    }
}

fn get_extension_for_type(file_type: &FileType) -> &'static str {
    match file_type {
        FileType::PlainText => "txt",
        FileType::Markdown => "md",
        FileType::Rust => "rs",
        FileType::Python => "py",
        FileType::JavaScript => "js",
        FileType::TypeScript => "ts",
        FileType::HTML => "html",
        FileType::CSS => "css",
        FileType::JSON => "json",
        FileType::XML => "xml",
        FileType::YAML => "yaml",
        FileType::TOML => "toml",
        FileType::SVG => "svg",
        FileType::Image(ext) => ext.as_str(),
        FileType::C => "c",
        FileType::CPP => "cpp",
        FileType::Java => "java",
        FileType::Go => "go",
    }
}

pub fn suggest_filename(content: &str, path: Option<&Path>, file_type: &FileType) -> String {
    if let Some(p) = path {
        if let Some(filename) = p.file_name().and_then(|f| f.to_str()) {
            return filename.to_string();
        }
    }
    
    let ext = get_extension_for_type(file_type);
    
    let suggested_name = extract_meaningful_name(content, file_type);
    
    format!("{}.{}", suggested_name, ext)
}

fn extract_meaningful_name(content: &str, file_type: &FileType) -> String {
    let lines: Vec<&str> = content.lines().take(20).collect();
    
    match file_type {
        FileType::Rust => {
            for line in &lines {
                if line.starts_with("fn ") {
                    if let Some(name) = line.strip_prefix("fn ") {
                        let name = name.split('(').next().unwrap_or("main");
                        return sanitize_name(name);
                    }
                }
            }
            "main".to_string()
        }
        FileType::Python => {
            for line in &lines {
                if line.starts_with("def ") {
                    if let Some(name) = line.strip_prefix("def ") {
                        let name = name.split('(').next().unwrap_or("main");
                        return sanitize_name(name);
                    }
                }
            }
            "main".to_string()
        }
        _ => "untitled".to_string(),
    }
}
