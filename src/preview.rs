use crate::file_type::FileType;
use crate::syntax_highlight::SyntaxHighlighter;
use egui::{Color32, FontId, RichText, Ui};
use pulldown_cmark::{html::push_html, Parser};
use std::sync::OnceLock;

static HIGHLIGHTER: OnceLock<SyntaxHighlighter> = OnceLock::new();

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
    let fontdb = usvg::fontdb::Database::new();
    
    match usvg::Tree::from_str(svg_content, &options, &fontdb) {
        Ok(tree) => {
            let size = tree.size();
            let width = size.width() as u32;
            let height = size.height() as u32;
            
            if width == 0 || height == 0 {
                ui.colored_label(Color32::RED, "SVG 尺寸无效");
                return;
            }
            
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
            
            let mut image = egui::Image::new(&texture);
            image = image.max_size(display_size);
            ui.add(image);
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
    let highlighter = HIGHLIGHTER.get_or_init(|| SyntaxHighlighter::new());
    
    let syntax_name = file_type.to_syntax_name();
    
    egui::ScrollArea::vertical().show(ui, |ui| {
        if let Some(syntax) = syntax_name {
            ui.label(RichText::new(format!("语法: {}", syntax)).color(Color32::GRAY));
        } else {
            ui.label(RichText::new("无语法高亮").color(Color32::GRAY));
        }
        ui.add_space(10.0);
        
        if let Some(syntax) = syntax_name {
            let lines = highlighter.highlight(text, syntax);
            for line in lines {
                ui.horizontal_wrapped(|ui| {
                    for segment in line.segments {
                        let label = RichText::new(&segment.text)
                            .font(FontId::monospace(14.0))
                            .color(segment.color);
                        ui.label(label);
                    }
                });
            }
        } else {
            for line in text.lines() {
                ui.label(RichText::new(line).font(FontId::monospace(14.0)));
            }
        }
    });
}
