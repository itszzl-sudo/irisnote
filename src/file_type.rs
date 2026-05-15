use std::path::Path;
use std::fmt;

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
    Kotlin,
    Swift,
    Ruby,
    PHP,
    Perl,
    Lua,
    Shell,
    PowerShell,
    SQL,
    Dockerfile,
    Makefile,
    CMake,
    Config,
    Mermaid,
    Dot,
    PlantUML,
    Markmap,
    WGSL,
    // 新增文件类型
    Log,
    CSV,
    TSV,
    RTF,
    TeX,
    Vue,
    Less,
    Gradle,
    Maven,
    Bazel,
    Patch,
    Diff,
    Certificate(String),
    Test,
    // 文档格式（阅读器模式）
    Word,
    PDF,
    Excel,
    PowerPoint,
    Unknown(String),
}

impl FileType {
    pub fn to_syntax_name(&self) -> Option<&'static str> {
        match self {
            FileType::Rust => Some("Rust"),
            FileType::Python => Some("Python"),
            FileType::JavaScript => Some("JavaScript"),
            FileType::TypeScript => Some("TypeScript"),
            FileType::HTML => Some("HTML"),
            FileType::CSS => Some("CSS"),
            FileType::JSON => Some("JSON"),
            FileType::XML => Some("XML"),
            FileType::YAML => Some("YAML"),
            FileType::TOML => Some("TOML"),
            FileType::C => Some("C"),
            FileType::CPP => Some("C++"),
            FileType::Java => Some("Java"),
            FileType::Go => Some("Go"),
            FileType::Kotlin => Some("Kotlin"),
            FileType::Swift => Some("Swift"),
            FileType::Ruby => Some("Ruby"),
            FileType::PHP => Some("PHP"),
            FileType::Perl => Some("Perl"),
            FileType::Lua => Some("Lua"),
            FileType::Shell => Some("Bash"),
            FileType::PowerShell => Some("PowerShell"),
            FileType::SQL => Some("SQL"),
            FileType::Dockerfile => Some("Dockerfile"),
            FileType::Makefile => Some("Makefile"),
            FileType::CMake => Some("CMake"),
            FileType::WGSL => Some("WGSL"),
            // 新增文件类型的语法高亮
            FileType::Vue => Some("Vue"),
            FileType::Less => Some("Less"),
            FileType::Gradle => Some("Groovy"),
            FileType::Maven => Some("XML"),
            FileType::Bazel => Some("Starlark"),
            FileType::Patch => Some("Diff"),
            FileType::Diff => Some("Diff"),
            FileType::TeX => Some("LaTeX"),
            _ => None,
        }
    }
    
    pub fn supports_preview(&self) -> bool {
        matches!(
            self,
            FileType::Markdown |
            FileType::SVG |
            FileType::Image(_) |
            FileType::Rust |
            FileType::Python |
            FileType::JavaScript |
            FileType::TypeScript |
            FileType::HTML |
            FileType::CSS |
            FileType::JSON |
            FileType::XML |
            FileType::YAML |
            FileType::TOML |
            FileType::C |
            FileType::CPP |
            FileType::Java |
            FileType::Go |
            FileType::Kotlin |
            FileType::Swift |
            FileType::Ruby |
            FileType::PHP |
            FileType::Perl |
            FileType::Lua |
            FileType::Shell |
            FileType::SQL |
            FileType::Dockerfile |
            FileType::Makefile |
            FileType::CMake |
            // 新增支持预览的文件类型
            FileType::Log |
            FileType::CSV |
            FileType::TSV |
            FileType::RTF |
            FileType::TeX |
            FileType::Vue |
            FileType::Less |
            FileType::Gradle |
            FileType::Maven |
            FileType::Bazel |
            FileType::Patch |
            FileType::Diff |
            FileType::Certificate(_) |
            FileType::Test
        )
    }

    pub fn has_visual_preview(&self) -> bool {
        matches!(
            self,
            FileType::Markdown | FileType::SVG | FileType::Image(_) |
            FileType::Mermaid | FileType::Dot | FileType::PlantUML | FileType::Markmap |
            FileType::Word | FileType::PDF | FileType::Excel | FileType::PowerPoint
        )
    }
}

pub fn get_supported_extensions() -> Vec<String> {
    vec![
        "txt".to_string(),
        "md".to_string(),
        "rst".to_string(),
        "rs".to_string(),
        "py".to_string(),
        "js".to_string(),
        "jsx".to_string(),
        "ts".to_string(),
        "tsx".to_string(),
        "html".to_string(),
        "htm".to_string(),
        "css".to_string(),
        "scss".to_string(),
        "sass".to_string(),
        "less".to_string(),
        "json".to_string(),
        "xml".to_string(),
        "yaml".to_string(),
        "yml".to_string(),
        "toml".to_string(),
        "ini".to_string(),
        "cfg".to_string(),
        "conf".to_string(),
        "properties".to_string(),
        "svg".to_string(),
        "c".to_string(),
        "h".to_string(),
        "cpp".to_string(),
        "cc".to_string(),
        "cxx".to_string(),
        "hpp".to_string(),
        "java".to_string(),
        "go".to_string(),
        "kt".to_string(),
        "kts".to_string(),
        "swift".to_string(),
        "sh".to_string(),
        "bash".to_string(),
        "zsh".to_string(),
        "fish".to_string(),
        "ps1".to_string(),
        "bat".to_string(),
        "cmd".to_string(),
        "sql".to_string(),
        "lua".to_string(),
        "rb".to_string(),
        "php".to_string(),
        "pl".to_string(),
        "pm".to_string(),
        "log".to_string(),
        "csv".to_string(),
        "tsv".to_string(),
        "env".to_string(),
        "gitignore".to_string(),
        "gitattributes".to_string(),
        "editorconfig".to_string(),
        "license".to_string(),
        "readme".to_string(),
        "changelog".to_string(),
        "authors".to_string(),
        "contributors".to_string(),
        "dockerfile".to_string(),
        "makefile".to_string(),
        "cmake".to_string(),
        "gradle".to_string(),
        "maven".to_string(),
        "mermaid".to_string(),
        "mmd".to_string(),
        "dot".to_string(),
        "gv".to_string(),
        "plantuml".to_string(),
        "puml".to_string(),
        "iuml".to_string(),
        "markmap".to_string(),
        "mm".to_string(),
        // 新增文件类型
        "markdown".to_string(),
        "vue".to_string(),
        "wgsl".to_string(),
        "rtf".to_string(),
        "tex".to_string(),
        "patch".to_string(),
        "diff".to_string(),
        "spec".to_string(),
        "test".to_string(),
        "feature".to_string(),
        "pem".to_string(),
        "cer".to_string(),
        "crt".to_string(),
        "key".to_string(),
        "pfx".to_string(),
        "bazel".to_string(),
        "bzl".to_string(),
        "lock".to_string(),
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
            "log" | "csv" | "tsv" | "env" | "properties" |
            "mermaid" | "mmd" | "dot" | "gv" | "plantuml" | "puml" | "iuml" |
            "markmap" | "mm" | "wgsl" |
            // 新增文件类型
            "markdown" | "vue" | "rtf" | "tex" | "patch" | "diff" |
            "spec" | "test" | "feature" | "pem" | "cer" | "crt" | "key" | "pfx" |
            "bazel" | "bzl" | "lock"
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
            ".bashrc" | ".zshrc" | ".profile" | "cargo.toml" | "package.json" |
            // 新增文件名
            "gemfile.lock" | "yarn.lock" | "package-lock.json"
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
                "css" | "scss" | "sass" => FileType::CSS,
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
                "kt" | "kts" => FileType::Kotlin,
                "swift" => FileType::Swift,
                "rb" => FileType::Ruby,
                "php" => FileType::PHP,
                "pl" | "pm" => FileType::Perl,
                "lua" => FileType::Lua,
                "sh" | "bash" | "zsh" => FileType::Shell,
                "ps1" | "psm1" => FileType::PowerShell,
                "sql" => FileType::SQL,
                "cmake" => FileType::CMake,
                "ini" | "cfg" | "conf" | "properties" => FileType::Config,
                "mermaid" | "mmd" => FileType::Mermaid,
                "dot" | "gv" => FileType::Dot,
                "plantuml" | "puml" | "iuml" => FileType::PlantUML,
                "markmap" | "mm" => FileType::Markmap,
                "wgsl" => FileType::WGSL,
                // 新增文件类型
                "markdown" => FileType::Markdown,
                "vue" => FileType::Vue,
                "less" => FileType::Less,
                "log" => FileType::Log,
                "csv" => FileType::CSV,
                "tsv" => FileType::TSV,
                "rtf" => FileType::RTF,
                "tex" => FileType::TeX,
                "gradle" => FileType::Gradle,
                "maven" => FileType::Maven,
                "bazel" | "bzl" => FileType::Bazel,
                "patch" => FileType::Patch,
                "diff" => FileType::Diff,
                "spec" | "test" | "feature" => FileType::Test,
                "pem" | "cer" | "crt" | "key" | "pfx" => FileType::Certificate(ext.to_string()),
                // 文档格式（阅读器模式）
                "docx" | "doc" | "dotx" => FileType::Word,
                "pdf" => FileType::PDF,
                "xlsx" | "xls" | "xlsm" | "xltx" | "xlt" => FileType::Excel,
                "pptx" | "ppt" | "pptm" | "potx" | "pot" => FileType::PowerPoint,
                _ => FileType::Unknown(ext.to_string()),
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
            if filename_lower == "makefile" {
                return FileType::Makefile;
            }
            if filename_lower == "dockerfile" {
                return FileType::Dockerfile;
            }
            if filename_lower == "cmakelists.txt" {
                return FileType::CMake;
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
        FileType::Kotlin => "kt".to_string(),
        FileType::Swift => "swift".to_string(),
        FileType::Ruby => "rb".to_string(),
        FileType::PHP => "php".to_string(),
        FileType::Perl => "pl".to_string(),
        FileType::Lua => "lua".to_string(),
        FileType::Shell => "sh".to_string(),
        FileType::PowerShell => "ps1".to_string(),
        FileType::SQL => "sql".to_string(),
        FileType::Dockerfile => "Dockerfile".to_string(),
        FileType::Makefile => "Makefile".to_string(),
        FileType::CMake => "cmake".to_string(),
        FileType::Config => "cfg".to_string(),
        FileType::Mermaid => "mmd".to_string(),
        FileType::Dot => "dot".to_string(),
        FileType::PlantUML => "puml".to_string(),
        FileType::Markmap => "mm".to_string(),
        FileType::WGSL => "wgsl".to_string(),
        // 新增文件类型
        FileType::Log => "log".to_string(),
        FileType::CSV => "csv".to_string(),
        FileType::TSV => "tsv".to_string(),
        FileType::RTF => "rtf".to_string(),
        FileType::TeX => "tex".to_string(),
        FileType::Vue => "vue".to_string(),
        FileType::Less => "less".to_string(),
        FileType::Gradle => "gradle".to_string(),
        FileType::Maven => "xml".to_string(),
        FileType::Bazel => "bzl".to_string(),
        FileType::Patch => "patch".to_string(),
        FileType::Diff => "diff".to_string(),
        FileType::Certificate(ext) => ext.clone(),
        FileType::Test => "test".to_string(),
        // 文档格式（阅读器模式）
        FileType::Word => "docx".to_string(),
        FileType::PDF => "pdf".to_string(),
        FileType::Excel => "xlsx".to_string(),
        FileType::PowerPoint => "pptx".to_string(),
        FileType::Unknown(ext) => ext.clone(),
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileType::PlainText => write!(f, "Plain Text"),
            FileType::Markdown => write!(f, "Markdown"),
            FileType::Rust => write!(f, "Rust"),
            FileType::Python => write!(f, "Python"),
            FileType::JavaScript => write!(f, "JavaScript"),
            FileType::TypeScript => write!(f, "TypeScript"),
            FileType::HTML => write!(f, "HTML"),
            FileType::CSS => write!(f, "CSS"),
            FileType::JSON => write!(f, "JSON"),
            FileType::XML => write!(f, "XML"),
            FileType::YAML => write!(f, "YAML"),
            FileType::TOML => write!(f, "TOML"),
            FileType::SVG => write!(f, "SVG"),
            FileType::Image(ext) => write!(f, "{}", ext.to_uppercase()),
            FileType::C => write!(f, "C"),
            FileType::CPP => write!(f, "C++"),
            FileType::Java => write!(f, "Java"),
            FileType::Go => write!(f, "Go"),
            FileType::Kotlin => write!(f, "Kotlin"),
            FileType::Swift => write!(f, "Swift"),
            FileType::Ruby => write!(f, "Ruby"),
            FileType::PHP => write!(f, "PHP"),
            FileType::Perl => write!(f, "Perl"),
            FileType::Lua => write!(f, "Lua"),
            FileType::Shell => write!(f, "Shell"),
            FileType::PowerShell => write!(f, "PowerShell"),
            FileType::SQL => write!(f, "SQL"),
            FileType::Dockerfile => write!(f, "Dockerfile"),
            FileType::Makefile => write!(f, "Makefile"),
            FileType::CMake => write!(f, "CMake"),
            FileType::Config => write!(f, "Config"),
            FileType::Mermaid => write!(f, "Mermaid"),
            FileType::Dot => write!(f, "Graphviz"),
            FileType::PlantUML => write!(f, "PlantUML"),
            FileType::Markmap => write!(f, "Markmap"),
            FileType::WGSL => write!(f, "WGSL"),
            // 新增文件类型
            FileType::Log => write!(f, "Log"),
            FileType::CSV => write!(f, "CSV"),
            FileType::TSV => write!(f, "TSV"),
            FileType::RTF => write!(f, "RTF"),
            FileType::TeX => write!(f, "LaTeX"),
            FileType::Vue => write!(f, "Vue"),
            FileType::Less => write!(f, "Less"),
            FileType::Gradle => write!(f, "Gradle"),
            FileType::Maven => write!(f, "Maven"),
            FileType::Bazel => write!(f, "Bazel"),
            FileType::Patch => write!(f, "Patch"),
            FileType::Diff => write!(f, "Diff"),
            FileType::Certificate(ext) => write!(f, "Certificate ({})", ext),
            FileType::Test => write!(f, "Test"),
            // 文档格式（阅读器模式）
            FileType::Word => write!(f, "Microsoft Word"),
            FileType::PDF => write!(f, "PDF"),
            FileType::Excel => write!(f, "Microsoft Excel"),
            FileType::PowerPoint => write!(f, "Microsoft PowerPoint"),
            FileType::Unknown(ext) => write!(f, "{}", ext),
        }
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

pub fn suggest_filenames(content: &str, path: Option<&Path>, file_type: &FileType) -> Vec<String> {
    // 如果已有路径且文件名有意义，返回空列表
    if let Some(p) = path {
        if let Some(filename) = p.file_name().and_then(|f| f.to_str()) {
            if !is_meaningless_filename(filename) {
                return Vec::new();
            }
        }
    }
    
    let ext = get_extension_for_type(file_type);
    let suggestions = extract_multiple_names(content, file_type);
    
    suggestions
        .into_iter()
        .map(|name| format!("{}.{}", name, ext))
        .filter(|name| name.len() <= 20)
        .take(5)
        .collect()
}

pub fn is_meaningless_filename(filename: &str) -> bool {
    let meaningless = ["untitled", "new file", "document", "file", "readme"];
    let name = filename.to_lowercase();
    meaningless.iter().any(|m| name.starts_with(m))
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

fn extract_multiple_names(content: &str, file_type: &FileType) -> Vec<String> {
    let lines: Vec<&str> = content.lines().take(20).collect();
    let mut suggestions = Vec::new();
    
    match file_type {
        FileType::Rust => {
            for line in &lines {
                if line.starts_with("fn ") {
                    if let Some(name) = line.strip_prefix("fn ") {
                        let name = name.split('(').next().unwrap_or("");
                        let sanitized = sanitize_name(name);
                        if !sanitized.is_empty() && !suggestions.contains(&sanitized) {
                            suggestions.push(sanitized);
                        }
                    }
                }
                if line.starts_with("mod ") {
                    if let Some(name) = line.strip_prefix("mod ") {
                        let name = name.trim().trim_end_matches(';');
                        let sanitized = sanitize_name(name);
                        if !sanitized.is_empty() && !suggestions.contains(&sanitized) {
                            suggestions.push(sanitized);
                        }
                    }
                }
            }
        }
        FileType::Python => {
            for line in &lines {
                if line.starts_with("def ") {
                    if let Some(name) = line.strip_prefix("def ") {
                        let name = name.split('(').next().unwrap_or("");
                        let sanitized = sanitize_name(name);
                        if !sanitized.is_empty() && !suggestions.contains(&sanitized) {
                            suggestions.push(sanitized);
                        }
                    }
                }
                if line.starts_with("class ") {
                    if let Some(name) = line.strip_prefix("class ") {
                        let name = name.split(':').next().unwrap_or("");
                        let sanitized = sanitize_name(name);
                        if !sanitized.is_empty() && !suggestions.contains(&sanitized) {
                            suggestions.push(sanitized);
                        }
                    }
                }
            }
        }
        FileType::JavaScript | FileType::TypeScript => {
            for line in &lines {
                if line.starts_with("function ") {
                    if let Some(name) = line.strip_prefix("function ") {
                        let name = name.split('(').next().unwrap_or("");
                        let sanitized = sanitize_name(name);
                        if !sanitized.is_empty() && !suggestions.contains(&sanitized) {
                            suggestions.push(sanitized);
                        }
                    }
                }
                if line.starts_with("class ") {
                    if let Some(name) = line.strip_prefix("class ") {
                        let name = name.split('{').next().unwrap_or("");
                        let sanitized = sanitize_name(name);
                        if !sanitized.is_empty() && !suggestions.contains(&sanitized) {
                            suggestions.push(sanitized);
                        }
                    }
                }
            }
        }
        FileType::Markdown => {
            for line in &lines {
                if line.starts_with("# ") {
                    if let Some(title) = line.strip_prefix("# ") {
                        let sanitized = sanitize_name(title);
                        if !sanitized.is_empty() && !suggestions.contains(&sanitized) {
                            suggestions.push(sanitized);
                        }
                    }
                }
            }
        }
        FileType::HTML => {
            if content.contains("<title>") {
                if let Some(start) = content.find("<title>") {
                    if let Some(end) = content.find("</title>") {
                        if end > start {
                            let title = &content[start + 7..end];
                            let sanitized = sanitize_name(title);
                            if !sanitized.is_empty() && !suggestions.contains(&sanitized) {
                                suggestions.push(sanitized);
                            }
                        }
                    }
                }
            }
        }
        FileType::Go => {
            for line in &lines {
                if line.starts_with("func ") {
                    if let Some(name) = line.strip_prefix("func ") {
                        let name = name.split('(').next().unwrap_or("");
                        let sanitized = sanitize_name(name);
                        if !sanitized.is_empty() && !suggestions.contains(&sanitized) {
                            suggestions.push(sanitized);
                        }
                    }
                }
            }
        }
        FileType::Java => {
            for line in &lines {
                if line.contains("class ") {
                    for part in line.split("class ") {
                        if !part.is_empty() {
                            let name = part.split_whitespace().next().unwrap_or("");
                            let sanitized = sanitize_name(name);
                            if !sanitized.is_empty() && !suggestions.contains(&sanitized) {
                                suggestions.push(sanitized);
                            }
                        }
                    }
                }
            }
        }
        FileType::JSON => {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(content) {
                if let Some(obj) = json.as_object() {
                    if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
                        let sanitized = sanitize_name(name);
                        if !sanitized.is_empty() && !suggestions.contains(&sanitized) {
                            suggestions.push(sanitized);
                        }
                    }
                    if let Some(name) = obj.get("package").and_then(|v| v.as_str()) {
                        let sanitized = sanitize_name(name);
                        if !sanitized.is_empty() && !suggestions.contains(&sanitized) {
                            suggestions.push(sanitized);
                        }
                    }
                }
            }
        }
        FileType::TOML => {
            for line in &lines {
                if line.starts_with("name = ") {
                    if let Some(name) = line.strip_prefix("name = ") {
                        let sanitized = sanitize_name(name.trim_matches('"'));
                        if !sanitized.is_empty() && !suggestions.contains(&sanitized) {
                            suggestions.push(sanitized);
                        }
                    }
                }
            }
        }
        _ => {}
    }
    
    // 添加一些通用的备选名称
    if suggestions.is_empty() {
        suggestions.push("document".to_string());
        suggestions.push("notes".to_string());
        suggestions.push("file".to_string());
    }
    
    suggestions
}

#[derive(Debug, Clone)]
pub struct FileTypeCategory {
    pub name: String,
    pub extensions: Vec<String>,
    pub descriptions: Vec<String>,
}

pub fn get_file_type_categories() -> Vec<FileTypeCategory> {
    vec![
        FileTypeCategory {
            name: "📝 文档".to_string(),
            extensions: vec![".txt".to_string(), ".md".to_string(), ".rst".to_string()],
            descriptions: vec!["纯文本".to_string(), "Markdown".to_string(), "reStructuredText".to_string()],
        },
        FileTypeCategory {
            name: "💻 编程语言".to_string(),
            extensions: vec![
                ".rs".to_string(), ".py".to_string(), ".js".to_string(), ".ts".to_string(),
                ".java".to_string(), ".go".to_string(), ".kt".to_string(), ".swift".to_string(),
                ".rb".to_string(), ".php".to_string(), ".c".to_string(), ".cpp".to_string(),
                ".lua".to_string(), ".pl".to_string(),
            ],
            descriptions: vec![
                "Rust".to_string(), "Python".to_string(), "JavaScript".to_string(), "TypeScript".to_string(),
                "Java".to_string(), "Go".to_string(), "Kotlin".to_string(), "Swift".to_string(),
                "Ruby".to_string(), "PHP".to_string(), "C".to_string(), "C++".to_string(),
                "Lua".to_string(), "Perl".to_string(),
            ],
        },
        FileTypeCategory {
            name: "🌐 Web 技术".to_string(),
            extensions: vec![".html".to_string(), ".htm".to_string(), ".css".to_string(), ".scss".to_string()],
            descriptions: vec!["HTML".to_string(), "HTML".to_string(), "CSS".to_string(), "SCSS".to_string()],
        },
        FileTypeCategory {
            name: "📋 数据格式".to_string(),
            extensions: vec![".json".to_string(), ".xml".to_string(), ".yaml".to_string(), ".yml".to_string(), ".toml".to_string()],
            descriptions: vec!["JSON".to_string(), "XML".to_string(), "YAML".to_string(), "YAML".to_string(), "TOML".to_string()],
        },
        FileTypeCategory {
            name: "📊 图表".to_string(),
            extensions: vec![".mmd".to_string(), ".dot".to_string(), ".gv".to_string(), ".puml".to_string(), ".iuml".to_string(), ".mm".to_string()],
            descriptions: vec!["Mermaid".to_string(), "Graphviz DOT".to_string(), "Graphviz".to_string(), "PlantUML".to_string(), "PlantUML".to_string(), "Markmap".to_string()],
        },
        FileTypeCategory {
            name: "🎨 图像".to_string(),
            extensions: vec![".svg".to_string(), ".png".to_string(), ".jpg".to_string(), ".jpeg".to_string(), ".gif".to_string()],
            descriptions: vec!["SVG".to_string(), "PNG".to_string(), "JPEG".to_string(), "JPEG".to_string(), "GIF".to_string()],
        },
        FileTypeCategory {
            name: "⚙️ 配置".to_string(),
            extensions: vec![".ini".to_string(), ".cfg".to_string(), ".conf".to_string(), ".properties".to_string(), ".env".to_string()],
            descriptions: vec!["INI".to_string(), "Config".to_string(), "Config".to_string(), "Properties".to_string(), "Environment".to_string()],
        },
        FileTypeCategory {
            name: "🐚 Shell".to_string(),
            extensions: vec![".sh".to_string(), ".bash".to_string(), ".zsh".to_string(), ".fish".to_string(), ".ps1".to_string(), ".bat".to_string(), ".cmd".to_string()],
            descriptions: vec!["Bash".to_string(), "Bash".to_string(), "Zsh".to_string(), "Fish".to_string(), "PowerShell".to_string(), "Batch".to_string(), "Batch".to_string()],
        },
        FileTypeCategory {
            name: "🗄️ 数据库".to_string(),
            extensions: vec![".sql".to_string()],
            descriptions: vec!["SQL".to_string()],
        },
        FileTypeCategory {
            name: "🐳 容器".to_string(),
            extensions: vec!["dockerfile".to_string(), "makefile".to_string(), ".cmake".to_string()],
            descriptions: vec!["Dockerfile".to_string(), "Makefile".to_string(), "CMake".to_string()],
        },
        FileTypeCategory {
            name: "🎮 着色器".to_string(),
            extensions: vec![".wgsl".to_string()],
            descriptions: vec!["WGSL".to_string()],
        },
        FileTypeCategory {
            name: "📊 数据表格".to_string(),
            extensions: vec![".csv".to_string(), ".tsv".to_string(), ".log".to_string()],
            descriptions: vec!["CSV".to_string(), "TSV".to_string(), "日志文件".to_string()],
        },
        FileTypeCategory {
            name: "📝 富文档".to_string(),
            extensions: vec![".rtf".to_string(), ".tex".to_string()],
            descriptions: vec!["RTF".to_string(), "LaTeX".to_string()],
        },
        FileTypeCategory {
            name: "⚛️ Web 框架".to_string(),
            extensions: vec![".vue".to_string(), ".jsx".to_string(), ".tsx".to_string(), ".less".to_string()],
            descriptions: vec!["Vue.js".to_string(), "React JSX".to_string(), "React TSX".to_string(), "Less".to_string()],
        },
        FileTypeCategory {
            name: "🔧 构建工具".to_string(),
            extensions: vec![".gradle".to_string(), ".bzl".to_string(), "bazel".to_string()],
            descriptions: vec!["Gradle".to_string(), "Bazel".to_string(), "Bazel".to_string()],
        },
        FileTypeCategory {
            name: "🔄 版本控制".to_string(),
            extensions: vec![".patch".to_string(), ".diff".to_string(), ".lock".to_string()],
            descriptions: vec!["补丁文件".to_string(), "差异文件".to_string(), "依赖锁文件".to_string()],
        },
        FileTypeCategory {
            name: "🔐 证书".to_string(),
            extensions: vec![".pem".to_string(), ".cer".to_string(), ".crt".to_string(), ".key".to_string(), ".pfx".to_string()],
            descriptions: vec!["PEM证书".to_string(), "证书".to_string(), "证书".to_string(), "私钥".to_string(), "PKCS#12".to_string()],
        },
        FileTypeCategory {
            name: "🧪 测试".to_string(),
            extensions: vec![".test".to_string(), ".spec".to_string(), ".feature".to_string()],
            descriptions: vec!["测试文件".to_string(), "测试规范".to_string(), "BDD特性".to_string()],
        },
        FileTypeCategory {
            name: "📚 办公文档".to_string(),
            extensions: vec![".docx".to_string(), ".doc".to_string(), ".pdf".to_string(), ".xlsx".to_string(), ".xls".to_string(), ".pptx".to_string(), ".ppt".to_string()],
            descriptions: vec!["Word 文档".to_string(), "Word 文档".to_string(), "PDF 文件".to_string(), "Excel 表格".to_string(), "Excel 表格".to_string(), "PowerPoint 演示".to_string(), "PowerPoint 演示".to_string()],
        },
    ]
}

pub fn search_file_types(query: &str) -> Vec<(String, String)> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();
    
    let all_types = vec![
        (".txt", "纯文本文件"),
        (".md", "Markdown 文档"),
        (".rst", "reStructuredText"),
        (".rs", "Rust 源代码"),
        (".py", "Python 源代码"),
        (".js", "JavaScript 文件"),
        (".ts", "TypeScript 文件"),
        (".jsx", "React JSX"),
        (".tsx", "React TSX"),
        (".java", "Java 源代码"),
        (".go", "Go 源代码"),
        (".kt", "Kotlin 源代码"),
        (".swift", "Swift 源代码"),
        (".rb", "Ruby 源代码"),
        (".php", "PHP 脚本"),
        (".c", "C 源代码"),
        (".cpp", "C++ 源代码"),
        (".h", "C/C++ 头文件"),
        (".lua", "Lua 脚本"),
        (".pl", "Perl 脚本"),
        (".html", "HTML 文档"),
        (".htm", "HTML 文档"),
        (".css", "CSS 样式表"),
        (".scss", "SCSS 样式表"),
        (".json", "JSON 数据"),
        (".xml", "XML 文档"),
        (".yaml", "YAML 配置"),
        (".yml", "YAML 配置"),
        (".toml", "TOML 配置"),
        (".svg", "SVG 矢量图"),
        (".mmd", "Mermaid 图表"),
        (".dot", "Graphviz DOT"),
        (".puml", "PlantUML 图表"),
        (".iuml", "PlantUML 图表"),
        (".mm", "Markmap 思维导图"),
        (".wgsl", "WGSL 着色器"),
        (".sql", "SQL 查询"),
        (".sh", "Shell 脚本"),
        (".bash", "Bash 脚本"),
        (".zsh", "Zsh 脚本"),
        (".fish", "Fish 脚本"),
        (".ps1", "PowerShell 脚本"),
        (".bat", "Windows Batch"),
        (".cmd", "Windows Batch"),
        (".ini", "INI 配置"),
        (".cfg", "配置文件"),
        (".conf", "配置文件"),
        (".properties", "Java Properties"),
        (".env", "环境变量文件"),
        (".dockerfile", "Dockerfile"),
        (".makefile", "Makefile"),
        (".cmake", "CMake 文件"),
        // 新增文件类型
        (".markdown", "Markdown 文档"),
        (".vue", "Vue.js 组件"),
        (".less", "Less 样式表"),
        (".log", "日志文件"),
        (".csv", "CSV 数据"),
        (".tsv", "TSV 数据"),
        (".rtf", "RTF 文档"),
        (".tex", "LaTeX 文档"),
        (".gradle", "Gradle 构建"),
        (".bazel", "Bazel 构建"),
        (".bzl", "Bazel 扩展"),
        (".patch", "补丁文件"),
        (".diff", "差异文件"),
        (".spec", "测试规范"),
        (".test", "测试文件"),
        (".feature", "BDD 特性"),
        (".pem", "PEM 证书"),
        (".cer", "证书文件"),
        (".crt", "证书文件"),
        (".key", "私钥文件"),
        (".pfx", "PKCS#12 证书"),
        (".lock", "依赖锁文件"),
    ];
    
    for (ext, desc) in all_types {
        if ext.contains(&query_lower) || desc.to_lowercase().contains(&query_lower) {
            results.push((ext.to_string(), desc.to_string()));
        }
    }
    
    results.sort_by(|a, b| a.0.cmp(&b.0));
    results
}
