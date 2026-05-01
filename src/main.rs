mod file_type;
mod preview;
mod config;
mod file_association;
mod syntax_highlight;
mod bitnet_service;

use eframe::egui;
use egui::{Color32, FontId, RichText, Vec2};
use std::path::PathBuf;
use std::fs;
use crate::file_type::{detect_file_type, FileType, suggest_filename};
use crate::preview::{PreviewMode, render_preview};
use crate::config::Config;
use crate::bitnet_service::{BitNetService, BitNetConfig};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("IrisNote"),
        ..Default::default()
    };
    
    eframe::run_native(
        "IrisNote",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Box::new(TextEditor::new(cc))
        }),
    )
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    if let Ok(font_data) = std::fs::read("C:/Windows/Fonts/msyh.ttc") {
        fonts.font_data.insert(
            "msyh".to_owned(),
            egui::FontData::from_owned(font_data),
        );
        
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "msyh".to_owned());
        
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "msyh".to_owned());
        
        ctx.set_fonts(fonts);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tab {
    Editor,
    Preview,
}

struct TextEditor {
    text: String,
    file_path: Option<PathBuf>,
    file_type: FileType,
    config: Config,
    recent_paths: Vec<PathBuf>,
    message: Option<String>,
    
    current_tab: Tab,
    show_summary: bool,
    summary: Option<String>,
    suggested_filename: Option<String>,
    
    bitnet_service: std::sync::Arc<BitNetService>,
}

impl TextEditor {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config = Config::load().unwrap_or_default();
        let recent_paths = config.recent_paths.clone();
        let bitnet_config = BitNetConfig::default();
        let bitnet_service = std::sync::Arc::new(BitNetService::with_config(bitnet_config));
        
        Self {
            text: String::new(),
            file_path: None,
            file_type: FileType::PlainText,
            config,
            recent_paths,
            message: None,
            current_tab: Tab::Preview,
            show_summary: true,
            summary: None,
            suggested_filename: None,
            bitnet_service,
        }
    }
    
    fn update_file_type(&mut self) {
        self.file_type = detect_file_type(&self.text, self.file_path.as_deref());
        self.analyze_content();
    }
    
    fn analyze_content(&mut self) {
        if self.text.is_empty() {
            self.summary = None;
            self.suggested_filename = None;
            return;
        }
        
        let suggested = suggest_filename(&self.text, self.file_path.as_deref(), &self.file_type);
        self.suggested_filename = Some(suggested);
        
        match self.bitnet_service.summarize_content(&self.text) {
            Ok(summary) if summary != "untitled" => {
                self.summary = Some(summary);
            }
            _ => {
                self.summary = self.generate_summary();
            }
        }
    }
    
    fn generate_summary(&self) -> Option<String> {
        let lines: Vec<&str> = self.text.lines().take(50).collect();
        
        match &self.file_type {
            FileType::Rust | FileType::Python | FileType::JavaScript | 
            FileType::TypeScript | FileType::Java | FileType::Go | 
            FileType::C | FileType::CPP | FileType::Kotlin | FileType::Swift |
            FileType::Ruby | FileType::PHP | FileType::Perl | FileType::Lua |
            FileType::Shell | FileType::PowerShell => {
                for line in &lines {
                    let trimmed = line.trim();
                    if trimmed.starts_with("//") || trimmed.starts_with("#") || 
                       trimmed.starts_with("/*") || trimmed.starts_with("--") {
                        let comment = trimmed.trim_start_matches('/').trim_start_matches('#')
                                            .trim_start_matches('*').trim_start_matches('-').trim();
                        if !comment.is_empty() && comment.len() > 10 {
                            return Some(format!("用法: {}", comment));
                        }
                    }
                }
                
                let first_line = lines.first()?.trim();
                if first_line.starts_with("fn ") || first_line.starts_with("def ") ||
                   first_line.starts_with("function ") || first_line.starts_with("func ") {
                    return Some(format!("目标: {}", first_line));
                }
                
                Some("代码文件".to_string())
            }
            FileType::SQL => {
                for line in &lines {
                    let trimmed = line.trim();
                    if trimmed.starts_with("--") {
                        let comment = trimmed.trim_start_matches('-').trim();
                        if !comment.is_empty() {
                            return Some(format!("查询: {}", comment));
                        }
                    }
                }
                Some("SQL 脚本".to_string())
            }
            FileType::HTML | FileType::CSS => {
                if self.text.contains("<title>") {
                    if let Some(start) = self.text.find("<title>") {
                        if let Some(end) = self.text.find("</title>") {
                            if end > start {
                                return Some(format!("页面: {}", &self.text[start+7..end]));
                            }
                        }
                    }
                }
                Some("网页文件".to_string())
            }
            FileType::Markdown => {
                for line in &lines {
                    if line.starts_with("# ") {
                        return Some(format!("文档: {}", line.strip_prefix("# ").unwrap_or("")));
                    }
                }
                Some("Markdown 文档".to_string())
            }
            FileType::Dockerfile => Some("Docker 构建配置".to_string()),
            FileType::Makefile => Some("Make 构建脚本".to_string()),
            FileType::CMake => Some("CMake 构建配置".to_string()),
            _ => {
                let word_count = self.text.split_whitespace().count();
                let line_count = self.text.lines().count();
                Some(format!("{} 行, {} 词", line_count, word_count))
            }
        }
    }
    
    fn save_file(&mut self) {
        if let Some(path) = &self.file_path {
            if let Err(e) = fs::write(path, &self.text) {
                self.message = Some(format!("保存失败: {}", e));
            } else {
                self.message = Some("文件已保存".to_string());
                self.add_recent_path(path.clone());
            }
        } else {
            self.message = Some("请先选择保存位置".to_string());
        }
    }
    
    fn save_as(&mut self, path: PathBuf) {
        if let Err(e) = fs::write(&path, &self.text) {
            self.message = Some(format!("保存失败: {}", e));
        } else {
            self.file_path = Some(path.clone());
            self.message = Some("文件已保存".to_string());
            self.add_recent_path(path);
            self.update_file_type();
        }
    }
    
    fn add_recent_path(&mut self, path: PathBuf) {
        self.recent_paths.retain(|p| p != &path);
        self.recent_paths.insert(0, path);
        if self.recent_paths.len() > 10 {
            self.recent_paths.truncate(10);
        }
        self.config.recent_paths = self.recent_paths.clone();
        let _ = self.config.save();
    }
    
    fn open_file(&mut self, path: PathBuf) {
        match fs::read_to_string(&path) {
            Ok(content) => {
                self.text = content;
                self.file_path = Some(path.clone());
                self.add_recent_path(path);
                self.update_file_type();
                self.message = Some("文件已打开".to_string());
            }
            Err(e) => {
                self.message = Some(format!("打开失败: {}", e));
            }
        }
    }
    
    fn get_suggested_filename(&self) -> String {
        if let Some(ref name) = self.suggested_filename {
            name.clone()
        } else {
            suggest_filename(&self.text, self.file_path.as_deref(), &self.file_type)
        }
    }
    
    fn get_preview_mode(&self) -> PreviewMode {
        match &self.file_type {
            FileType::Markdown => PreviewMode::Markdown,
            FileType::SVG => PreviewMode::Image,
            FileType::Image(_) => PreviewMode::Image,
            _ if self.file_type.to_syntax_name().is_some() => PreviewMode::Highlighted,
            _ => PreviewMode::Editor,
        }
    }
}

impl eframe::App for TextEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("文件", |ui| {
                    if ui.button("新建").clicked() {
                        self.text.clear();
                        self.file_path = None;
                        self.file_type = FileType::PlainText;
                        self.summary = None;
                        self.suggested_filename = None;
                        ui.close_menu();
                    }
                    
                    if ui.button("打开...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.open_file(path);
                        }
                        ui.close_menu();
                    }
                    
                    ui.menu_button("最近打开", |ui| {
                        let paths: Vec<PathBuf> = self.recent_paths.clone();
                        for path in paths.iter() {
                            if ui.button(path.display().to_string()).clicked() {
                                self.open_file(path.clone());
                                ui.close_menu();
                            }
                        }
                    });
                    
                    ui.separator();
                    
                    if ui.button("保存").clicked() {
                        self.save_file();
                        ui.close_menu();
                    }
                    
                    if ui.button("另存为...").clicked() {
                        let suggested = self.get_suggested_filename();
                        if let Some(path) = rfd::FileDialog::new()
                            .set_file_name(&suggested)
                            .save_file()
                        {
                            self.save_as(path);
                        }
                        ui.close_menu();
                    }
                });
                
                #[cfg(target_os = "windows")]
                ui.menu_button("工具", |ui| {
                    if ui.button("关联文件类型").clicked() {
                        file_association::register_all_extensions();
                        self.message = Some("文件关联已注册".to_string());
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("帮助", |ui| {
                    if ui.button("关于 IrisNote").clicked() {
                        ui.close_menu();
                    }
                    
                    ui.separator();
                    ui.label("IrisNote v0.1.0");
                    ui.label("智能文本编辑器");
                    ui.label("支持 50+ 文件类型");
                    ui.label("语法高亮 | 预览 | 智能分析");
                    
                    ui.separator();
                    ui.hyperlink_to("GitHub", "https://github.com/itszzl-sudo/irisnote");
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.small_button("⚙").clicked() {
                        self.show_summary = !self.show_summary;
                    }
                });
            });
        });
        
        if self.show_summary && (self.summary.is_some() || self.suggested_filename.is_some()) {
            egui::TopBottomPanel::top("summary_bar")
                .resizable(false)
                .show_separator_line(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if let Some(ref summary) = self.summary {
                            ui.label(RichText::new("📝 ").color(Color32::LIGHT_BLUE));
                            ui.label(RichText::new(summary).color(Color32::LIGHT_GRAY));
                            ui.separator();
                        }
                        
                        if let Some(ref filename) = self.suggested_filename {
                            ui.label(RichText::new("📁 ").color(Color32::LIGHT_GREEN));
                            ui.label(RichText::new(format!("建议: {}", filename)).color(Color32::LIGHT_GRAY));
                        }
                    });
                });
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, Tab::Editor, "编辑器");
                ui.selectable_value(&mut self.current_tab, Tab::Preview, "预览");
            });
            ui.separator();
            
            match self.current_tab {
                Tab::Editor => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let mut text = self.text.clone();
                        let available = ui.available_size();
                        ui.add(
                            egui::TextEdit::multiline(&mut text)
                                .desired_width(available.x - 10.0)
                                .desired_rows(40)
                                .font(FontId::monospace(14.0))
                        );
                        if text != self.text {
                            self.text = text;
                            self.update_file_type();
                        }
                    });
                }
                Tab::Preview => {
                    let preview_mode = self.get_preview_mode();
                    render_preview(ui, &self.text, &self.file_type, &preview_mode);
                }
            }
        });
        
        if let Some(msg) = &self.message {
            egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(msg).color(Color32::GREEN));
                });
            });
        }
    }
}
