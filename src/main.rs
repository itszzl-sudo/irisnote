mod file_type;
mod preview;
mod config;
mod file_association;

#[cfg(feature = "bitnet")]
mod bitnet_service;

use eframe::egui;
use egui::{Color32, FontId, RichText, Vec2};
use std::path::PathBuf;
use std::fs;
use crate::file_type::{detect_file_type, FileType, suggest_filename};
use crate::preview::{PreviewMode, render_preview};
use crate::config::Config;

#[cfg(feature = "bitnet")]
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
        Box::new(|cc| Box::new(TextEditor::new(cc))),
    )
}

struct TextEditor {
    text: String,
    file_path: Option<PathBuf>,
    file_type: FileType,
    preview_mode: PreviewMode,
    config: Config,
    recent_paths: Vec<PathBuf>,
    show_preview: bool,
    message: Option<String>,
    
    #[cfg(feature = "bitnet")]
    bitnet_service: std::sync::Arc<BitNetService>,
    
    #[cfg(feature = "bitnet")]
    bitnet_enabled: bool,
    
    #[cfg(feature = "bitnet")]
    suggested_filename: Option<String>,
}

impl TextEditor {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config = Config::load().unwrap_or_default();
        let recent_paths = config.recent_paths.clone();
        
        #[cfg(feature = "bitnet")]
        {
            let bitnet_config = BitNetConfig::default();
            let bitnet_enabled = bitnet_config.enabled;
            let bitnet_service = std::sync::Arc::new(BitNetService::with_config(bitnet_config));
            
            Self {
                text: String::new(),
                file_path: None,
                file_type: FileType::PlainText,
                preview_mode: PreviewMode::Editor,
                config,
                recent_paths,
                show_preview: false,
                message: None,
                bitnet_service,
                bitnet_enabled,
                suggested_filename: None,
            }
        }
        
        #[cfg(not(feature = "bitnet"))]
        {
            Self {
                text: String::new(),
                file_path: None,
                file_type: FileType::PlainText,
                preview_mode: PreviewMode::Editor,
                config,
                recent_paths,
                show_preview: false,
                message: None,
            }
        }
    }
    
    fn update_file_type(&mut self) {
        self.file_type = detect_file_type(&self.text, self.file_path.as_deref());
        self.preview_mode = match self.file_type {
            FileType::Markdown => PreviewMode::Markdown,
            FileType::SVG => PreviewMode::Image,
            FileType::JSON | FileType::XML => PreviewMode::Highlighted,
            FileType::Image(_) => PreviewMode::Image,
            _ => PreviewMode::Editor,
        };
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
    
    #[cfg(feature = "bitnet")]
    fn get_suggested_filename(&mut self) -> String {
        if let Some(ref name) = self.suggested_filename {
            return name.clone();
        }
        
        let traditional_name = suggest_filename(&self.text, self.file_path.as_deref(), &self.file_type);
        
        if traditional_name != "untitled.txt" && !traditional_name.starts_with("untitled.") {
            return traditional_name;
        }
        
        if self.bitnet_enabled {
            match self.bitnet_service.summarize_content(&self.text) {
                Ok(summary) if summary != "untitled" => {
                    let ext = self.get_extension();
                    let filename = format!("{}.{}", summary, ext);
                    self.suggested_filename = Some(filename.clone());
                    return filename;
                }
                _ => {}
            }
        }
        
        traditional_name
    }
    
    #[cfg(not(feature = "bitnet"))]
    fn get_suggested_filename(&mut self) -> String {
        suggest_filename(&self.text, self.file_path.as_deref(), &self.file_type)
    }
    
    #[cfg(feature = "bitnet")]
    fn get_extension(&self) -> String {
        match &self.file_type {
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
    
    #[cfg(feature = "bitnet")]
    fn summarize_with_bitnet(&mut self) {
        self.message = Some("正在分析内容...".to_string());
        
        match self.bitnet_service.summarize_content(&self.text) {
            Ok(summary) => {
                let ext = self.get_extension();
                let filename = format!("{}.{}", summary, ext);
                self.suggested_filename = Some(filename.clone());
                self.message = Some(format!("建议文件名: {}", filename));
            }
            Err(e) => {
                self.message = Some(format!("分析失败: {}", e));
            }
        }
    }
}

impl eframe::App for TextEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::menu::bar(ctx, |ui| {
            ui.menu_button("文件", |ui| {
                if ui.button("新建").clicked() {
                    self.text.clear();
                    self.file_path = None;
                    self.file_type = FileType::PlainText;
                    
                    #[cfg(feature = "bitnet")]
                    {
                        self.suggested_filename = None;
                    }
                    
                    ui.close_menu();
                }
                
                if ui.button("打开...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.open_file(path);
                    }
                    ui.close_menu();
                }
                
                ui.menu_button("最近打开", |ui| {
                    for path in self.recent_paths.iter() {
                        if ui.button(path.display().to_string()).clicked() {
                            self.open_file(path.clone());
                            ui.close_menu();
                        }
                    }
                });
                
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
            
            ui.menu_button("视图", |ui| {
                ui.checkbox(&mut self.show_preview, "显示预览");
                ui.separator();
                
                if ui.radio_value(&mut self.preview_mode, PreviewMode::Editor, "编辑器").clicked() {
                    ui.close_menu();
                }
                if ui.radio_value(&mut self.preview_mode, PreviewMode::Markdown, "Markdown 预览").clicked() {
                    ui.close_menu();
                }
                if ui.radio_value(&mut self.preview_mode, PreviewMode::Image, "图片预览").clicked() {
                    ui.close_menu();
                }
                if ui.radio_value(&mut self.preview_mode, PreviewMode::Highlighted, "语法高亮").clicked() {
                    ui.close_menu();
                }
            });
            
            #[cfg(feature = "bitnet")]
            ui.menu_button("智能分析", |ui| {
                ui.checkbox(&mut self.bitnet_enabled, "启用智能命名");
                
                if self.bitnet_enabled {
                    ui.separator();
                    
                    if ui.button("分析当前内容").clicked() {
                        self.summarize_with_bitnet();
                        ui.close_menu();
                    }
                    
                    if let Some(ref name) = self.suggested_filename {
                        ui.separator();
                        ui.label(format!("建议文件名: {}", name));
                    }
                }
                
                ui.separator();
                ui.label("基于关键词提取");
                ui.label("无需外部依赖");
            });
            
            #[cfg(not(feature = "bitnet"))]
            ui.menu_button("智能分析", |ui| {
                ui.label("智能分析未启用");
                ui.separator();
                ui.label("要启用请使用:");
                ui.label("cargo build --features bitnet");
            });
            
            #[cfg(target_os = "windows")]
            ui.menu_button("工具", |ui| {
                if ui.button("关联所有支持的文件类型").clicked() {
                    file_association::register_all_extensions();
                    self.message = Some("文件关联已注册".to_string());
                    ui.close_menu();
                }
                
                ui.menu_button("选择性关联", |ui| {
                    let extensions = file_type::get_supported_extensions();
                    for ext in extensions {
                        if ui.button(&format!(".{}", ext)).clicked() {
                            file_association::register_extension(&ext);
                            self.message = Some(format!(".{} 已关联", ext));
                            ui.close_menu();
                        }
                    }
                });
            });
        });
        
        if let Some(msg) = &self.message {
            egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(msg).color(Color32::GREEN));
                    
                    #[cfg(feature = "bitnet")]
                    if self.bitnet_enabled {
                        ui.separator();
                        ui.label(RichText::new("🔍 智能分析").color(Color32::LIGHT_BLUE));
                    }
                });
            });
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            let available = ui.available_size();
            
            if self.show_preview && self.preview_mode != PreviewMode::Editor {
                ui.horizontal(|ui| {
                    let editor_width = available.x * 0.5;
                    
                    ui.allocate_ui_with_layout(
                        Vec2::new(editor_width, available.y),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                let mut text = self.text.clone();
                                ui.add(
                                    egui::TextEdit::multiline(&mut text)
                                        .desired_width(editor_width - 10.0)
                                        .desired_rows(30)
                                );
                                if text != self.text {
                                    self.text = text;
                                    self.update_file_type();
                                    
                                    #[cfg(feature = "bitnet")]
                                    {
                                        self.suggested_filename = None;
                                    }
                                }
                            });
                        },
                    );
                    
                    ui.separator();
                    
                    render_preview(ui, &self.text, &self.file_type, &self.preview_mode);
                });
            } else {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let mut text = self.text.clone();
                    ui.add(
                        egui::TextEdit::multiline(&mut text)
                            .desired_width(available.x - 10.0)
                            .desired_rows(40)
                            .font(FontId::monospace(14.0))
                    );
                    if text != self.text {
                        self.text = text;
                        self.update_file_type();
                        
                        #[cfg(feature = "bitnet")]
                        {
                            self.suggested_filename = None;
                        }
                    }
                });
            }
        });
    }
}
