use crate::file_type::FileType;
use crate::syntax_highlight::SyntaxHighlighter;
use egui::{Color32, FontId, RichText, Ui, Context};
use pulldown_cmark::{html::push_html, Parser};
use std::sync::OnceLock;
use std::process::Command;
use std::path::PathBuf;

static HIGHLIGHTER: OnceLock<SyntaxHighlighter> = OnceLock::new();

pub fn handle_dropped_files<F>(_ctx: &Context, _handler: F)
where
    F: FnMut(&[PathBuf]),
{
    // 拖拽处理简化版本，因为不同 egui 版本 API 不同
    // TODO: 根据实际 egui 版本调整
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreviewMode {
    Editor,
    Markdown,
    Image,
    Highlighted,
    Diagram,
    Reader,
}

pub fn render_preview(ui: &mut Ui, text: &str, file_type: &FileType, mode: &PreviewMode) {
    match mode {
        PreviewMode::Markdown => render_markdown(ui, text),
        PreviewMode::Image => render_image(ui, text, file_type),
        PreviewMode::Highlighted => render_highlighted(ui, text, file_type),
        PreviewMode::Diagram => render_diagram(ui, text, file_type),
        PreviewMode::Reader => render_reader(ui, file_type),
        PreviewMode::Editor => {
            ui.label("编辑模式");
        }
    }
}

fn render_markdown(ui: &mut Ui, text: &str) {
    egui::ScrollArea::vertical()
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
        .show(ui, |ui| {
        ui.vertical(|ui| {
            let lines: Vec<&str> = text.lines().collect();
            let mut i = 0;
            
            while i < lines.len() {
                let line = lines[i];
                let trimmed = line.trim();
                
                if trimmed.starts_with("```") {
                    let mut code_lines = Vec::new();
                    let language = trimmed.trim_start_matches("```").trim().to_lowercase();
                    i += 1;
                    
                    while i < lines.len() && !lines[i].trim().starts_with("```") {
                        code_lines.push(lines[i]);
                        i += 1;
                    }
                    
                    render_code_block(ui, &code_lines, &language);
                    i += 1;
                    continue;
                }
                
                if trimmed.starts_with("# ") {
                    let title = trimmed.strip_prefix("# ").unwrap_or("");
                    ui.label(RichText::new(title).font(FontId::proportional(24.0)).strong());
                    ui.add_space(5.0);
                    i += 1;
                    continue;
                }
                
                if trimmed.starts_with("## ") {
                    let title = trimmed.strip_prefix("## ").unwrap_or("");
                    ui.label(RichText::new(title).font(FontId::proportional(20.0)).strong());
                    ui.add_space(5.0);
                    i += 1;
                    continue;
                }
                
                if trimmed.starts_with("### ") {
                    let title = trimmed.strip_prefix("### ").unwrap_or("");
                    ui.label(RichText::new(title).font(FontId::proportional(18.0)).strong());
                    ui.add_space(3.0);
                    i += 1;
                    continue;
                }
                
                if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                    let item = if trimmed.starts_with("- ") {
                        trimmed.strip_prefix("- ").unwrap_or("")
                    } else {
                        trimmed.strip_prefix("* ").unwrap_or("")
                    };
                    ui.horizontal(|ui| {
                        ui.label("•");
                        render_inline_markdown(ui, item);
                    });
                    i += 1;
                    continue;
                }
                
                if trimmed.starts_with("---") || trimmed.starts_with("***") {
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);
                    i += 1;
                    continue;
                }
                
                if trimmed.starts_with("> ") {
                    let quote = trimmed.strip_prefix("> ").unwrap_or("");
                    ui.horizontal(|ui| {
                        ui.colored_label(Color32::GRAY, "|");
                        ui.add_space(5.0);
                        ui.label(RichText::new(quote).italics().color(Color32::GRAY));
                    });
                    i += 1;
                    continue;
                }
                
                if trimmed.is_empty() {
                    ui.add_space(5.0);
                    i += 1;
                    continue;
                }
                
                render_inline_markdown(ui, trimmed);
                i += 1;
            }
        });
    });
}

fn render_code_block(ui: &mut Ui, lines: &[&str], language: &str) {
    ui.add_space(5.0);
    
    let bg_color = Color32::from_rgb(30, 30, 30);
    let (rect, response) = ui.allocate_at_least(
        egui::vec2(ui.available_width(), 0.0),
        egui::Sense::hover(),
    );
    
    if response.hovered() {
        ui.painter().rect_filled(rect.expand(5.0), 5.0, Color32::from_rgb(40, 40, 40));
    } else {
        ui.painter().rect_filled(rect.expand(5.0), 5.0, bg_color);
    }
    
    let inner_rect = rect.expand(-3.0);
    ui.painter().rect_stroke(inner_rect, 3.0, (1.0, Color32::from_rgb(60, 60, 60)));
    
    // 过滤行号后的代码文本（支持多种行号格式）
    let code_text = filter_line_numbers(lines).join("\n");
    
    // 右上角语言标签和复制按钮
    ui.allocate_ui_at_rect(egui::Rect::from_min_size(
        egui::pos2(inner_rect.left(), inner_rect.top()),
        egui::vec2(inner_rect.width(), 25.0),
    ), |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // 复制按钮（复制时过滤行号）
            if ui.button("📋 复制").clicked() {
                ui.ctx().copy_text(code_text.clone());
            }
            
            if !language.is_empty() {
                let lang_label = format!("{} ", language.to_uppercase());
                ui.label(RichText::new(lang_label).size(10.0).color(Color32::from_rgb(100, 150, 200)));
            }
        });
    });
    
    ui.set_min_height(30.0 + lines.len() as f32 * 18.0);
    
    let highlighted = highlight_code(&code_text, language);
    
    ui.painter().text(
        egui::pos2(inner_rect.left() + 10.0, inner_rect.top() + 30.0),
        egui::Align2::LEFT_TOP,
        &highlighted,
        FontId::monospace(13.0),
        Color32::WHITE,
    );
    
    // 当代码行数超过一定数量（模拟跨页）时，在右下角也显示复制按钮
    if lines.len() > 20 {
        ui.allocate_ui_at_rect(egui::Rect::from_min_size(
            egui::pos2(inner_rect.left(), inner_rect.bottom() - 25.0),
            egui::vec2(inner_rect.width(), 25.0),
        ), |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // 复制按钮（复制时过滤行号）
                if ui.button("📋 复制").clicked() {
                    ui.ctx().copy_text(code_text.clone());
                }
            });
        });
    }
    
    ui.add_space(10.0);
}

/// 过滤代码行中的行号
/// 支持的行号格式：
/// - 数字+冒号："1: fn main()" -> "fn main()"
/// - 数字+点："1. fn main()" -> "fn main()"  
/// - 数字+制表符/空格："1\tfn main()" 或 "1   fn main()" -> "fn main()"
fn filter_line_numbers(lines: &[&str]) -> Vec<String> {
    lines
        .iter()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let mut i = 0;
            
            // 跳过开头的空白字符
            while i < chars.len() && chars[i].is_whitespace() {
                i += 1;
            }
            
            // 检查是否有数字序列
            let start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            
            // 如果找到数字序列
            if i > start {
                // 检查数字后面是否有分隔符（: . \t 空格）
                if i < chars.len() && (chars[i] == ':' || chars[i] == '.' || chars[i].is_whitespace()) {
                    // 跳过分隔符和后续空白
                    while i < chars.len() && (chars[i] == ':' || chars[i] == '.' || chars[i].is_whitespace()) {
                        i += 1;
                    }
                    // 返回行号后的内容
                    return chars[i..].iter().collect();
                }
            }
            
            // 没有行号，返回原行
            line.to_string()
        })
        .collect()
}

fn highlight_code(code: &str, language: &str) -> String {
    let syntax_name = match language {
        "rs" | "rust" => "Rust",
        "py" | "python" => "Python",
        "js" | "javascript" => "JavaScript",
        "ts" | "typescript" => "TypeScript",
        "html" | "htm" => "HTML",
        "css" => "CSS",
        "json" => "JSON",
        "xml" => "XML",
        "yaml" | "yml" => "YAML",
        "toml" => "TOML",
        "c" => "C",
        "cpp" | "c++" => "C++",
        "java" => "Java",
        "go" => "Go",
        "kt" | "kotlin" => "Kotlin",
        "swift" => "Swift",
        "rb" | "ruby" => "Ruby",
        "php" => "PHP",
        "lua" => "Lua",
        "sh" | "bash" | "shell" => "Bash",
        "ps1" | "powershell" => "PowerShell",
        "sql" => "SQL",
        "wgsl" => "WGSL",
        _ => "Plain Text",
    };
    
    use syntect::parsing::SyntaxSet;
    use syntect::highlighting::{ThemeSet, Theme};
    use syntect::easy::HighlightLines;
    use syntect::util::LinesWithEndings;
    
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let theme: &Theme = &theme_set.themes["base16-ocean.dark"];
    
    let syntax = syntax_set.find_syntax_by_name(syntax_name)
        .or_else(|| syntax_set.find_syntax_by_extension(language))
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
    
    let mut h = HighlightLines::new(syntax, theme);
    let mut result = String::new();
    
    for line in LinesWithEndings::from(code) {
        let ranges = h.highlight_line(line, &syntax_set).unwrap_or_default();
        for (style, text) in ranges {
            let fg = style.foreground;
            let color = format!("#{:02x}{:02x}{:02x}", fg.r, fg.g, fg.b);
            result.push_str(&format!("{}{}{}", "\x1b[38;2;", color, text.replace(']', "]]")));
        }
    }
    
    result
}

fn render_inline_markdown(ui: &mut Ui, text: &str) {
    let mut processed = String::new();
    let mut chars = text.chars().peekable();
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
            ui.label(
                RichText::new(&code)
                    .font(FontId::monospace(12.0))
                    .color(Color32::from_rgb(255, 180, 120))
                    .background_color(Color32::from_rgb(50, 50, 50))
            );
        } else {
            processed.push(c);
        }
    }
    
    if !processed.is_empty() {
        ui.label(&processed);
    }
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
    
    egui::ScrollArea::vertical()
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
        .show(ui, |ui| {
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

fn render_diagram(ui: &mut Ui, text: &str, file_type: &FileType) {
    let diagram_type = match file_type {
        FileType::Mermaid => "Mermaid",
        FileType::Dot => "Graphviz DOT",
        FileType::PlantUML => "PlantUML",
        FileType::Markmap => "Markmap",
        _ => "图表",
    };
    
    egui::ScrollArea::vertical()
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
        .show(ui, |ui| {
        ui.label(RichText::new(format!("{} 图表预览", diagram_type)).font(FontId::proportional(18.0)).strong());
        ui.add_space(10.0);
        
        match file_type {
            FileType::Mermaid => render_mermaid(ui, text),
            FileType::Dot => render_dot(ui, text),
            FileType::PlantUML => render_plantuml(ui, text),
            FileType::Markmap => render_markmap(ui, text),
            _ => {
                ui.label(RichText::new("图表源代码:").strong());
                ui.add_space(5.0);
                ui.label(RichText::new(text).font(FontId::monospace(12.0)).color(Color32::LIGHT_GRAY));
            }
        }
    });
}

fn render_mermaid(ui: &mut Ui, text: &str) {
    // 先尝试用 mermaid-text 渲染 Unicode 文本图表
    if let Ok(text_output) = mermaid_text::render(text) {
        ui.label(RichText::new("📊 Unicode 文本图表:").strong());
        ui.add_space(5.0);
        ui.label(RichText::new(text_output).font(FontId::monospace(12.0)));
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);
    }
    
    // 尝试用 mermaid-rs-renderer 渲染 SVG
    ui.label(RichText::new("🎨 SVG 渲染尝试:").strong());
    ui.add_space(5.0);
    
    match mermaid_rs_renderer::render(text) {
        Ok(svg) => {
            // 渲染 SVG 为图像
            if let Err(e) = render_svg_from_string(ui, &svg) {
                ui.colored_label(Color32::RED, format!("SVG 渲染失败: {}", e));
            }
        }
        Err(e) => {
            ui.colored_label(Color32::RED, format!("Mermaid 解析失败: {}", e));
            ui.add_space(10.0);
            ui.label(RichText::new("图表源代码:").strong());
            ui.add_space(5.0);
            ui.label(RichText::new(text).font(FontId::monospace(12.0)).color(Color32::LIGHT_GRAY));
        }
    }
}

fn render_dot(ui: &mut Ui, text: &str) {
    ui.label(RichText::new("📊 Graphviz DOT 图表:").strong());
    ui.add_space(5.0);
    
    // 尝试调用系统 dot 命令
    let result = || -> Result<String, Box<dyn std::error::Error>> {
        use std::process::Stdio;
        
        let mut child = Command::new("dot")
            .args(&["-Tsvg", "-o-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        
        if let Some(mut stdin) = child.stdin.take() {
            std::io::Write::write_all(&mut stdin, text.as_bytes())?;
        }
        
        let output = child.wait_with_output()?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into());
        }
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }();
    
    match result {
        Ok(svg) => {
            if let Err(e) = render_svg_from_string(ui, &svg) {
                ui.colored_label(Color32::RED, format!("SVG 渲染失败: {}", e));
            }
        }
        Err(e) => {
            ui.colored_label(Color32::YELLOW, format!("未找到 dot 命令或执行失败 (请安装 Graphviz): {}", e));
            ui.add_space(10.0);
            ui.label(RichText::new("💡 提示: 请安装 Graphviz 以获得更好的渲染效果").color(Color32::GRAY));
            ui.add_space(10.0);
            show_fallback_text(ui, text);
        }
    }
}

fn render_plantuml(ui: &mut Ui, text: &str) {
    ui.label(RichText::new("📊 PlantUML 图表:").strong());
    ui.add_space(5.0);
    
    // 尝试调用 plantuml.jar 或 plantuml 命令
    let mut success = false;
    
    // 尝试 plantuml 命令
    let try_plantuml = || -> Result<String, Box<dyn std::error::Error>> {
        use std::process::Stdio;
        
        let mut child = Command::new("plantuml")
            .args(&["-tsvg", "-pipe"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        
        if let Some(mut stdin) = child.stdin.take() {
            std::io::Write::write_all(&mut stdin, text.as_bytes())?;
        }
        
        let output = child.wait_with_output()?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into());
        }
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    };
    
    let try_java_plantuml = || -> Result<String, Box<dyn std::error::Error>> {
        use std::process::Stdio;
        
        let mut child = Command::new("java")
            .args(&["-jar", "plantuml.jar", "-tsvg", "-pipe"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        
        if let Some(mut stdin) = child.stdin.take() {
            std::io::Write::write_all(&mut stdin, text.as_bytes())?;
        }
        
        let output = child.wait_with_output()?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into());
        }
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    };
    
    if let Ok(svg) = try_plantuml() {
        if render_svg_from_string(ui, &svg).is_ok() {
            success = true;
        }
    }
    
    if !success {
        if let Ok(svg) = try_java_plantuml() {
            if render_svg_from_string(ui, &svg).is_ok() {
                success = true;
            }
        }
    }
    
    if !success {
        ui.colored_label(Color32::YELLOW, "未找到 plantuml 命令或 plantuml.jar");
        ui.add_space(10.0);
        ui.label(RichText::new("💡 提示: 请安装 PlantUML 或配置 plantuml.jar").color(Color32::GRAY));
        ui.add_space(10.0);
        show_fallback_text(ui, text);
    }
}

fn render_markmap(ui: &mut Ui, text: &str) {
    ui.label(RichText::new("📊 Markmap 思维导图:").strong());
    ui.add_space(5.0);
    ui.colored_label(Color32::YELLOW, "Markmap 渲染需要 JavaScript 环境");
    ui.add_space(10.0);
    ui.label(RichText::new("💡 提示: Markmap 需要浏览器环境渲染").color(Color32::GRAY));
    ui.add_space(10.0);
    show_fallback_text(ui, text);
}

fn show_fallback_text(ui: &mut Ui, text: &str) {
    ui.label(RichText::new("图表源代码:").strong());
    ui.add_space(5.0);
    ui.label(RichText::new(text).font(FontId::monospace(12.0)).color(Color32::LIGHT_GRAY));
}

fn render_svg_from_string(ui: &mut Ui, svg: &str) -> Result<(), Box<dyn std::error::Error>> {
    let options = usvg::Options::default();
    let fontdb = usvg::fontdb::Database::new();
    let tree = usvg::Tree::from_str(svg, &options, &fontdb)?;
    
    let size = tree.size();
    let width = size.width() as u32;
    let height = size.height() as u32;
    
    if width == 0 || height == 0 {
        ui.colored_label(Color32::RED, "SVG 尺寸无效");
        return Ok(());
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
        "diagram_preview",
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
    
    Ok(())
}

fn render_reader(ui: &mut Ui, file_type: &FileType) {
    egui::ScrollArea::vertical()
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
        .show(ui, |ui| {
            match file_type {
                FileType::Word => {
                    ui.label(RichText::new("📄 Microsoft Word 文档").font(FontId::proportional(20.0)).strong());
                    ui.add_space(10.0);
                    ui.colored_label(Color32::BLUE, "📖 阅读器模式");
                    ui.add_space(15.0);
                    
                    ui.group(|ui| {
                        ui.label(RichText::new("文件格式").strong());
                        ui.label("Microsoft Word (.docx, .doc)");
                    });
                    ui.add_space(10.0);
                    
                    ui.group(|ui| {
                        ui.label(RichText::new("功能特点").strong());
                        ui.label("✓ 支持 .docx 和 .doc 格式");
                        ui.label("✓ 只读预览模式");
                        ui.label("✓ 文本提取与显示");
                        ui.label("✓ 段落结构展示");
                    });
                    ui.add_space(10.0);
                    
                    ui.colored_label(Color32::YELLOW, "💡 提示: 双击文件在默认应用中打开以进行编辑");
                    ui.add_space(5.0);
                    ui.label(RichText::new("文档内容将以纯文本形式显示").color(Color32::GRAY));
                }
                FileType::PDF => {
                    ui.label(RichText::new("📄 PDF 文档").font(FontId::proportional(20.0)).strong());
                    ui.add_space(10.0);
                    ui.colored_label(Color32::GREEN, "📖 阅读器模式");
                    ui.add_space(15.0);
                    
                    ui.group(|ui| {
                        ui.label(RichText::new("文件格式").strong());
                        ui.label("Portable Document Format (.pdf)");
                    });
                    ui.add_space(10.0);
                    
                    ui.group(|ui| {
                        ui.label(RichText::new("功能特点").strong());
                        ui.label("✓ PDF 文件预览");
                        ui.label("✓ 页面导航支持");
                        ui.label("✓ 文本搜索功能");
                        ui.label("✓ 缩放与打印");
                    });
                    ui.add_space(10.0);
                    
                    ui.colored_label(Color32::YELLOW, "💡 提示: 双击文件在默认 PDF 阅读器中打开以获得完整功能");
                }
                FileType::Excel => {
                    ui.label(RichText::new("📊 Microsoft Excel 表格").font(FontId::proportional(20.0)).strong());
                    ui.add_space(10.0);
                    ui.colored_label(Color32::from_rgb(255, 165, 0), "📖 阅读器模式");
                    ui.add_space(15.0);
                    
                    ui.group(|ui| {
                        ui.label(RichText::new("文件格式").strong());
                        ui.label("Microsoft Excel (.xlsx, .xls)");
                    });
                    ui.add_space(10.0);
                    
                    ui.group(|ui| {
                        ui.label(RichText::new("功能特点").strong());
                        ui.label("✓ 支持 .xlsx 和 .xls 格式");
                        ui.label("✓ 表格数据展示");
                        ui.label("✓ 单元格格式保留");
                        ui.label("✓ 公式计算结果");
                    });
                    ui.add_space(10.0);
                    
                    ui.colored_label(Color32::YELLOW, "💡 提示: 双击文件在 Excel 中打开以进行编辑");
                    ui.add_space(5.0);
                    ui.label(RichText::new("表格数据将以结构化形式显示").color(Color32::GRAY));
                }
                FileType::PowerPoint => {
                    ui.label(RichText::new("🎬 Microsoft PowerPoint 演示").font(FontId::proportional(20.0)).strong());
                    ui.add_space(10.0);
                    ui.colored_label(Color32::from_rgb(128, 0, 128), "📖 阅读器模式");
                    ui.add_space(15.0);
                    
                    ui.group(|ui| {
                        ui.label(RichText::new("文件格式").strong());
                        ui.label("Microsoft PowerPoint (.pptx, .ppt)");
                    });
                    ui.add_space(10.0);
                    
                    ui.group(|ui| {
                        ui.label(RichText::new("功能特点").strong());
                        ui.label("✓ 支持 .pptx 和 .ppt 格式");
                        ui.label("✓ 幻灯片预览");
                        ui.label("✓ 演讲者备注");
                        ui.label("✓ 动画效果预览");
                    });
                    ui.add_space(10.0);
                    
                    ui.colored_label(Color32::YELLOW, "💡 提示: 双击文件在 PowerPoint 中打开以进行演示和编辑");
                }
                _ => {
                    ui.label(RichText::new("📖 阅读器模式").font(FontId::proportional(18.0)).strong());
                    ui.add_space(10.0);
                    ui.label("该文件类型不支持阅读器模式");
                }
            }
        });
}
