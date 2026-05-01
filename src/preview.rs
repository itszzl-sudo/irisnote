use crate::file_type::FileType;
use egui::{Color32, FontId, RichText, TextureHandle, Ui, Vec2};
use pulldown_cmark::{html::push_html, Parser};

#[derive(Debug, Clone, PartialEq)]
pub enum PreviewMode {
    Editor,
    Markdown,
    Image,
    Highlighted,
}

pub fn render_preview(ui: &mut Ui, text: &str, file_type: &FileType, mode: &PreviewMode) {
    match mode {
        PreviewMode::Markdown => render_markdown(ui, text),
        PreviewMode::Image => render_image(ui, text, file_type),
        PreviewMode::Highlighted => render_highlighted(ui, text, file_type),
        PreviewMode::Editor => {
            ui.label("编辑模式");
        }
    }
}

fn render_markdown(ui: &mut Ui, text: &str) {
    let parser = Parser::new(text);
    let mut html_output = String::new();
    push_html(&mut html_output, parser);
    
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.vertical(|ui| {
            let lines = text.lines();
            let mut in_code_block = false;
            let mut in_list = false;
            
            for line in lines {
                let trimmed = line.trim();
                
                if trimmed.starts_with("```") {
                    in_code_block = !in_code_block;
                    if in_code_block {
                        ui.add_space(5.0);
                    }
                    continue;
                }
                
                if in_code_block {
                    ui.label(RichText::new(line).font(FontId::monospace(12.0)).color(Color32::LIGHT_GRAY));
                    continue;
                }
                
                if trimmed.starts_with("# ") {
                    let title = trimmed.strip_prefix("# ").unwrap_or("");
                    ui.label(RichText::new(title).font(FontId::proportional(24.0)).strong());
                    ui.add_space(5.0);
                    continue;
                }
                
                if trimmed.starts_with("## ") {
                    let title = trimmed.strip_prefix("## ").unwrap_or("");
                    ui.label(RichText::new(title).font(FontId::proportional(20.0)).strong());
                    ui.add_space(5.0);
                    continue;
                }
                
                if trimmed.starts_with("### ") {
                    let title = trimmed.strip_prefix("### ").unwrap_or("");
                    ui.label(RichText::new(title).font(FontId::proportional(18.0)).strong());
                    ui.add_space(3.0);
                    continue;
                }
                
                if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                    in_list = true;
                    let item = if trimmed.starts_with("- ") {
                        trimmed.strip_prefix("- ").unwrap_or("")
                    } else {
                        trimmed.strip_prefix("* ").unwrap_or("")
                    };
                    ui.horizontal(|ui| {
                        ui.label("•");
                        ui.label(item);
                    });
                    continue;
                }
                
                if trimmed.starts_with(|c: char| c.is_numeric()) && trimmed.contains(". ") {
                    in_list = true;
                    ui.label(trimmed);
                    continue;
                }
                
                if in_list && trimmed.is_empty() {
                    in_list = false;
                    ui.add_space(5.0);
                    continue;
                }
                
                if trimmed.starts_with("**") && trimmed.ends_with("**") {
                    let text = trimmed.trim_matches('*');
                    ui.label(RichText::new(text).strong());
                    continue;
                }
                
                if trimmed.starts_with("*") && trimmed.ends_with("*") && !trimmed.starts_with("**") {
                    let text = trimmed.trim_matches('*');
                    ui.label(RichText::new(text).italics());
                    continue;
                }
                
                if trimmed.starts_with("---") || trimmed.starts_with("***") {
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);
                    continue;
                }
                
                if trimmed.starts_with("> ") {
                    let quote = trimmed.strip_prefix("> ").unwrap_or("");
                    ui.horizontal(|ui| {
                        ui.colored_label(Color32::GRAY, "| ");
                        ui.colored_label(Color32::GRAY, quote);
                    });
                    continue;
                }
                
                if !trimmed.is_empty() {
                    let mut processed = String::new();
                    let mut chars = line.chars().peekable();
                    let mut in_bold = false;
                    let mut in_italic = false;
                    
                    while let Some(c) = chars.next() {
                        if c == '*' {
                            if chars.peek() == Some(&'*') {
                                chars.next();
                                if in_bold {
                                    ui.label(RichText::new(&processed).strong());
                                    processed.clear();
                                }
                                in_bold = !in_bold;
                            } else if !in_bold {
                                if in_italic {
                                    ui.label(RichText::new(&processed).italics());
                                    processed.clear();
                                }
                                in_italic = !in_italic;
                            } else {
                                processed.push(c);
                            }
                        } else if c == '`' {
                            ui.label(&processed);
                            processed.clear();
                            let mut code = String::new();
                            while let Some(&next) = chars.peek() {
                                if next == '`' {
                                    chars.next();
                                    break;
                                }
                                code.push(chars.next().unwrap());
                            }
                            ui.label(RichText::new(&code).font(FontId::monospace(12.0)).color(Color32::LIGHT_BLUE));
                        } else {
                            processed.push(c);
                        }
                    }
                    
                    if !processed.is_empty() {
                        ui.label(&processed);
                    }
                } else {
                    ui.add_space(5.0);
                }
            }
        });
    });
}

fn render_image(ui: &mut Ui, text: &str, file_type: &FileType) {
    match file_type {
        FileType::SVG => render_svg(ui, text),
        FileType::Image(ref ext) => {
            ui.label(format!("图片格式: {}", ext));
            ui.label("无法直接显示二进制图片，请使用文件打开功能");
        }
        _ => {
            ui.label("无图片内容");
        }
    }
}

fn render_svg(ui: &mut Ui, svg_content: &str) {
    let options = usvg::Options::default();
    
    match usvg::Tree::from_str(svg_content, &options) {
        Ok(tree) => {
            let size = tree.size();
            let width = size.width() as u32;
            let height = size.height() as u32;
            
            let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
            
            resvg::render(
                &tree,
                usvg::Transform::identity(),
                &mut pixmap.as_mut(),
            );
            
            let pixels = pixmap.data();
            
            let color_image = egui::ColorImage {
                size: [width as usize, height as usize],
                pixels: pixels
                    .chunks_exact(4)
                    .map(|p| {
                        egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3])
                    })
                    .collect(),
            };
            
            let texture = ui.ctx().load_texture(
                "svg_preview",
                color_image,
                egui::TextureOptions::default(),
            );
            
            let available_size = ui.available_size();
            let texture_size = texture.size_vec2();
            
            let scale = (available_size.x / texture_size.x)
                .min(available_size.y / texture_size.y)
                .min(1.0);
            
            let display_size = texture_size * scale;
            
            ui.image(&texture).fit_to_exact_size(display_size);
        }
        Err(e) => {
            ui.colored_label(Color32::RED, format!("SVG 解析错误: {}", e));
            ui.add_space(10.0);
            ui.label("SVG 源代码:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(RichText::new(svg_content).font(FontId::monospace(12.0)));
            });
        }
    }
}

fn render_highlighted(ui: &mut Ui, text: &str, file_type: &FileType) {
    let syntax = match file_type {
        FileType::Rust => "rust",
        FileType::Python => "python",
        FileType::JavaScript => "javascript",
        FileType::TypeScript => "typescript",
        FileType::HTML => "html",
        FileType::CSS => "css",
        FileType::JSON => "json",
        FileType::XML => "xml",
        FileType::YAML => "yaml",
        FileType::TOML => "toml",
        FileType::C => "c",
        FileType::CPP => "cpp",
        FileType::Java => "java",
        FileType::Go => "go",
        _ => "text",
    };
    
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.label(RichText::new(format!("语法: {}", syntax)).color(Color32::GRAY));
        ui.add_space(10.0);
        
        for line in text.lines() {
            let colored_line = apply_simple_highlighting(line, file_type);
            ui.horizontal(|ui| {
                for (text, color) in colored_line {
                    ui.label(RichText::new(text).font(FontId::monospace(12.0)).color(color));
                }
            });
        }
    });
}

fn apply_simple_highlighting(line: &str, file_type: &FileType) -> Vec<(String, Color32)> {
    let mut result = Vec::new();
    
    let keywords = match file_type {
        FileType::Rust => vec!["fn", "let", "mut", "if", "else", "match", "struct", "enum", "impl", "pub", "use", "mod", "const", "static", "trait", "type", "where", "for", "while", "loop", "return", "break", "continue", "self", "Self"],
        FileType::Python => vec!["def", "class", "if", "elif", "else", "for", "while", "import", "from", "as", "return", "yield", "with", "try", "except", "finally", "raise", "lambda", "and", "or", "not", "in", "is", "True", "False", "None", "self"],
        FileType::JavaScript | FileType::TypeScript => vec!["function", "const", "let", "var", "if", "else", "for", "while", "do", "switch", "case", "break", "continue", "return", "class", "extends", "new", "this", "super", "import", "export", "from", "async", "await", "try", "catch", "finally", "throw"],
        FileType::Java => vec!["public", "private", "protected", "class", "interface", "extends", "implements", "new", "this", "super", "static", "final", "void", "return", "if", "else", "for", "while", "do", "switch", "case", "break", "continue", "try", "catch", "finally", "throw", "throws", "import", "package"],
        FileType::Go => vec!["package", "import", "func", "var", "const", "type", "struct", "interface", "map", "chan", "if", "else", "for", "range", "switch", "case", "default", "break", "continue", "return", "go", "defer", "select"],
        _ => vec![],
    };
    
    if keywords.is_empty() {
        result.push((line.to_string(), Color32::WHITE));
        return result;
    }
    
    let mut current = String::new();
    let mut in_string = false;
    let mut in_comment = false;
    let mut string_char = ' ';
    
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        let c = chars[i];
        
        if in_comment {
            current.push(c);
            i += 1;
            continue;
        }
        
        if in_string {
            current.push(c);
            if c == '\\' && i + 1 < chars.len() {
                i += 1;
                current.push(chars[i]);
            } else if c == string_char {
                result.push((current.clone(), Color32::GREEN));
                current.clear();
                in_string = false;
            }
            i += 1;
            continue;
        }
        
        if c == '"' || c == '\'' {
            if !current.is_empty() {
                result.push((current.clone(), Color32::WHITE));
                current.clear();
            }
            in_string = true;
            string_char = c;
            current.push(c);
            i += 1;
            continue;
        }
        
        if c == '/' && i + 1 < chars.len() {
            if chars[i + 1] == '/' {
                if !current.is_empty() {
                    result.push((current.clone(), Color32::WHITE));
                    current.clear();
                }
                let comment: String = chars[i..].iter().collect();
                result.push((comment, Color32::GRAY));
                break;
            }
        }
        
        if c == '#' && (file_type == &FileType::Python || file_type == &FileType::Rust) {
            if !current.is_empty() {
                result.push((current.clone(), Color32::WHITE));
                current.clear();
            }
            let comment: String = chars[i..].iter().collect();
            result.push((comment, Color32::GRAY));
            break;
        }
        
        if c.is_whitespace() || c == '(' || c == ')' || c == '{' || c == '}' || c == '[' || c == ']' || c == ';' || c == ',' || c == '.' || c == ':' || c == '=' || c == '+' || c == '-' || c == '*' || c == '/' || c == '<' || c == '>' || c == '!' || c == '&' || c == '|' {
            if !current.is_empty() {
                let color = if keywords.contains(&current.as_str()) {
                    Color32::from_rgb(255, 121, 198)
                } else if current.starts_with(char::is_uppercase) && file_type == &FileType::Rust {
                    Color32::from_rgb(102, 217, 239)
                } else if current.parse::<f64>().is_ok() {
                    Color32::from_rgb(249, 145, 87)
                } else {
                    Color32::WHITE
                };
                result.push((current.clone(), color));
                current.clear();
            }
            result.push((c.to_string(), Color32::WHITE));
        } else {
            current.push(c);
        }
        
        i += 1;
    }
    
    if !current.is_empty() {
        let color = if keywords.contains(&current.as_str()) {
            Color32::from_rgb(255, 121, 198)
        } else if current.starts_with(char::is_uppercase) && file_type == &FileType::Rust {
            Color32::from_rgb(102, 217, 239)
        } else if current.parse::<f64>().is_ok() {
            Color32::from_rgb(249, 145, 87)
        } else {
            Color32::WHITE
        };
        result.push((current, color));
    }
    
    result
}
