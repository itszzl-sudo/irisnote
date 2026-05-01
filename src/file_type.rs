use std::path::Path;

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

pub fn is_text_file(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext_lower = ext.to_lowercase();
        return matches!(
            ext_lower.as_str(),
            "txt" | "md" | "rst" | "rs" | "py" | "js" | "ts" | "jsx" | "tsx" |
            "html" | "htm" | "css" | "scss" | "sass" | "less" |
            "json" | "xml" | "yaml" | "yml" | "toml" | "ini" | "cfg" | "conf" |
            "svg" | "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" |
            "java" | "go" | "kt" | "kts" | "swift" |
            "sh" | "bash" | "zsh" | "fish" | "ps1" | "bat" | "cmd" |
            "sql" | "lua" | "rb" | "php" | "pl" | "pm" |
            "dockerfile" | "makefile" | "cmake" | "gradle" | "maven" |
            "gitignore" | "gitattributes" | "editorconfig" |
            "license" | "readme" | "changelog" | "authors" | "contributors" |
            "log" | "csv" | "tsv" | "env" | "properties"
        );
    }
    
    if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
        let filename_lower = filename.to_lowercase();
        return matches!(
            filename_lower.as_str(),
            "dockerfile" | "makefile" | "cmakelists.txt" | "vagrantfile" |
            "gemfile" | "rakefile" | "procfile" | "license" | "readme" |
            "changelog" | "authors" | "contributors" | "copying" |
            ".gitignore" | ".gitattributes" | ".editorconfig" | ".env" |
            ".bashrc" | ".zshrc" | ".profile" | "cargo.toml" | "package.json"
        );
    }
    
    false
}

pub fn detect_file_type(content: &str, path: Option<&Path>) -> FileType {
    if let Some(p) = path {
        if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
            return match ext.to_lowercase().as_str() {
                "md" => FileType::Markdown,
                "rs" => FileType::Rust,
                "py" => FileType::Python,
                "js" | "jsx" => FileType::JavaScript,
                "ts" | "tsx" => FileType::TypeScript,
                "html" | "htm" => FileType::HTML,
                "css" | "scss" | "sass" | "less" => FileType::CSS,
                "json" => FileType::JSON,
                "xml" => FileType::XML,
                "yaml" | "yml" => FileType::YAML,
                "toml" => FileType::TOML,
                "svg" => FileType::SVG,
                "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" => FileType::Image(ext.to_string()),
                "c" | "h" => FileType::C,
                "cpp" | "cc" | "cxx" | "hpp" => FileType::CPP,
                "java" => FileType::Java,
                "go" => FileType::Go,
                _ => detect_from_content(content),
            };
        }
        
        if let Some(filename) = p.file_name().and_then(|f| f.to_str()) {
            let filename_lower = filename.to_lowercase();
            if filename_lower == "cargo.toml" {
                return FileType::TOML;
            }
            if filename_lower == "package.json" {
                return FileType::JSON;
            }
            if filename_lower == "makefile" || filename_lower == "dockerfile" {
                return FileType::PlainText;
            }
        }
    }
    
    detect_from_content(content)
}

fn detect_from_content(content: &str) -> FileType {
    let trimmed = content.trim();
    
    if trimmed.is_empty() {
        return FileType::PlainText;
    }
    
    if trimmed.starts_with("<?xml") || (trimmed.starts_with('<') && trimmed.contains("xmlns")) {
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
    
    if trimmed.starts_with("public class") || (trimmed.starts_with("class ") && trimmed.contains("public static void main")) {
        return FileType::Java;
    }
    
    if trimmed.starts_with("function ") || trimmed.starts_with("const ") || 
       trimmed.starts_with("let ") || trimmed.contains("=>") {
        if trimmed.contains(": ") || trimmed.contains("interface ") {
            return FileType::TypeScript;
        }
        return FileType::JavaScript;
    }
    
    if trimmed.starts_with('.') || (trimmed.starts_with('#') && trimmed.contains('{')) {
        return FileType::CSS;
    }
    
    if trimmed.contains("# ") || trimmed.contains("## ") || 
       trimmed.contains("* ") || trimmed.contains("```") {
        return FileType::Markdown;
    }
    
    if trimmed.starts_with('[') && trimmed.contains('=') {
        return FileType::TOML;
    }
    
    FileType::PlainText
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

fn get_extension_for_type(file_type: &FileType) -> String {
    match file_type {
        FileType::PlainText => "txt".to_string(),
        FileType::Markdown => "md".to_string(),
        FileType::Rust => "rs".to_string(),
        FileType::Python => "py".to_string(),
        FileType::JavaScript => "js".to_string(),
        FileType::TypeScript => "ts".to_string(),
        FileType::HTML => "html".to_string(),
        FileType::CSS => "css".to_string(),
        FileType::JSON => "json".to_string(),
        FileType::XML => "xml".to_string(),
        FileType::YAML => "yaml".to_string(),
        FileType::TOML => "toml".to_string(),
        FileType::SVG => "svg".to_string(),
        FileType::Image(ext) => ext.clone(),
        FileType::C => "c".to_string(),
        FileType::CPP => "cpp".to_string(),
        FileType::Java => "java".to_string(),
        FileType::Go => "go".to_string(),
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
                if line.starts_with("mod ") {
                    if let Some(name) = line.strip_prefix("mod ") {
                        let name = name.trim().trim_end_matches(';');
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
                if line.starts_with("class ") {
                    if let Some(name) = line.strip_prefix("class ") {
                        let name = name.split(':').next().unwrap_or("MyClass");
                        return sanitize_name(name);
                    }
                }
            }
            "main".to_string()
        }
        FileType::JavaScript | FileType::TypeScript => {
            for line in &lines {
                if line.starts_with("function ") {
                    if let Some(name) = line.strip_prefix("function ") {
                        let name = name.split('(').next().unwrap_or("main");
                        return sanitize_name(name);
                    }
                }
            }
            "main".to_string()
        }
        FileType::Markdown => {
            for line in &lines {
                if line.starts_with("# ") {
                    if let Some(title) = line.strip_prefix("# ") {
                        return sanitize_name(title);
                    }
                }
            }
            "untitled".to_string()
        }
        FileType::HTML => {
            if content.contains("<title>") {
                if let Some(start) = content.find("<title>") {
                    if let Some(end) = content.find("</title>") {
                        if end > start {
                            let title = &content[start + 7..end];
                            return sanitize_name(title);
                        }
                    }
                }
            }
            "untitled".to_string()
        }
        FileType::Go => {
            for line in &lines {
                if line.starts_with("func ") {
                    if let Some(name) = line.strip_prefix("func ") {
                        let name = name.split('(').next().unwrap_or("main");
                        return sanitize_name(name);
                    }
                }
            }
            "main".to_string()
        }
        FileType::Java => {
            for line in &lines {
                if line.contains("class ") {
                    for part in line.split("class ") {
                        if !part.is_empty() {
                            let name = part.split_whitespace().next().unwrap_or("Main");
                            return sanitize_name(name);
                        }
                    }
                }
            }
            "Main".to_string()
        }
        FileType::JSON => {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(content) {
                if let Some(obj) = json.as_object() {
                    if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
                        return sanitize_name(name);
                    }
                    if let Some(name) = obj.get("package").and_then(|v| v.as_str()) {
                        return sanitize_name(name);
                    }
                }
            }
            "untitled".to_string()
        }
        FileType::TOML => {
            for line in &lines {
                if line.starts_with("name = ") {
                    if let Some(name) = line.strip_prefix("name = ") {
                        return sanitize_name(name.trim_matches('"'));
                    }
                }
            }
            "untitled".to_string()
        }
        _ => "untitled".to_string(),
    }
}
