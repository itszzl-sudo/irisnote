#![windows_subsystem = "windows"]

mod file_type;
mod preview;
mod config;
mod file_association;
mod syntax_highlight;
mod bitnet_service;

use eframe::egui;
use egui::{Color32, FontId, RichText};
use egui_extras::syntax_highlighting::{highlight, CodeTheme};
use std::path::PathBuf;
use std::sync::OnceLock;
use std::fs;
use crate::file_type::{detect_file_type, FileType, suggest_filename, suggest_filenames, is_meaningless_filename};
use crate::preview::{PreviewMode, render_preview};
use crate::config::Config;
use crate::bitnet_service::{BitNetService, BitNetConfig};

static CODE_THEME_LIGHT: OnceLock<CodeTheme> = OnceLock::new();
static CODE_THEME_DARK: OnceLock<CodeTheme> = OnceLock::new();

fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let initial_file_path = if args.len() > 1 {
        Some(PathBuf::from(&args[1]))
    } else {
        None
    };

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
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Box::new(TextEditor::new(cc, initial_file_path))
        }),
    )
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    // Load Microsoft YaHei for Chinese text
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
    }
    
    // Load Segoe UI Emoji for colored emojis
    if let Ok(emoji_data) = std::fs::read("C:/Windows/Fonts/seguisym.ttf") {
        fonts.font_data.insert(
            "emoji".to_owned(),
            egui::FontData::from_owned(emoji_data),
        );
        
        // Add emoji font as fallback for all families
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .push("emoji".to_owned());
        
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("emoji".to_owned());
    }
    
    ctx.set_fonts(fonts);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ViewMode {
    Editor,
    Preview,
    Split,
    FileTypes,
}

// 为了保持兼容性
type Tab = ViewMode;

struct TextEditor {
    text: String,
    file_path: Option<PathBuf>,
    file_type: FileType,
    config: Config,
    recent_paths: Vec<PathBuf>,
    message: Option<String>,
    
    current_tab: Tab,
    
    bitnet_service: std::sync::Arc<BitNetService>,
    is_dark_theme: bool,
    
    // 光标位置信息
    last_text_len: usize,
    
    // 文件类型搜索
    file_type_search_query: String,
    file_type_search_results: Vec<(String, String)>,
    
    // 保存时的命名建议
    show_save_dialog: bool,
    save_suggestions: Vec<String>,
    save_filename_input: String,
    message_timeout: Option<std::time::Instant>,
}

impl TextEditor {
    fn new(_cc: &eframe::CreationContext<'_>, initial_file_path: Option<PathBuf>) -> Self {
        let config = Config::load().unwrap_or_default();
        let recent_paths = config.recent_paths.clone();
        let bitnet_config = BitNetConfig::default();
        let bitnet_service = std::sync::Arc::new(BitNetService::with_config(bitnet_config));

        let mut editor = Self {
            text: String::new(),
            file_path: None,
            file_type: FileType::PlainText,
            config,
            recent_paths,
            message: None,
            current_tab: Tab::Editor,
            bitnet_service,
            is_dark_theme: false,
            last_text_len: 0,
            file_type_search_query: String::new(),
            file_type_search_results: Vec::new(),
            show_save_dialog: false,
            save_suggestions: Vec::new(),
            save_filename_input: String::new(),
            message_timeout: None,
        };

        if let Some(path) = initial_file_path {
            if path.exists() && path.is_file() {
                editor.open_file(path);
            }
        }

        editor
    }

    fn update_file_type(&mut self) {
        self.file_type = detect_file_type(&self.text, self.file_path.as_deref());
        // 内容分析已移除，现在只在保存时提供命名建议

        if self.file_type.has_visual_preview() && self.current_tab == Tab::Editor {
            // 如果是支持预览的文件，默认使用分屏模式而不是纯预览
            self.current_tab = Tab::Split;
        }
    }
    
    fn get_cursor_position(&self) -> (usize, usize) {
        // 简化版本：暂时返回合理位置
        let line_count = self.text.lines().count().max(1);
        let col = 1;
        (line_count, col)
    }

    fn save_file(&mut self) {
        if let Some(path) = &self.file_path {
            // 检查文件名是否有意义
            if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                if is_meaningless_filename(filename) {
                    // 如果文件名无意义，显示命名建议
                    self.save_suggestions = suggest_filenames(&self.text, Some(path), &self.file_type);
                    if !self.save_suggestions.is_empty() {
                        // 初始化文件名输入框，默认使用第一个建议
                        self.save_filename_input = self.save_suggestions[0].clone();
                        self.show_save_dialog = true;
                        return;
                    }
                }
            }
            // 直接保存
            self.do_save(path.clone());
        } else {
            // 新建文件，显示命名建议
            self.save_suggestions = suggest_filenames(&self.text, None, &self.file_type);
            if !self.save_suggestions.is_empty() {
                // 初始化文件名输入框，默认使用第一个建议
                self.save_filename_input = self.save_suggestions[0].clone();
                self.show_save_dialog = true;
            } else {
                self.message = Some("请先选择保存位置".to_string());
            }
        }
    }
    
    fn do_save(&mut self, path: PathBuf) {
        if let Err(e) = fs::write(&path, &self.text) {
            self.message = Some(format!("保存失败: {}", e));
        } else {
            self.file_path = Some(path.clone());
            self.message = Some("文件已保存".to_string());
            self.message_timeout = Some(std::time::Instant::now() + std::time::Duration::from_secs(3));
            self.add_recent_path(path);
            self.update_file_type();
        }
    }
    
    fn handle_save_with_suggestion(&mut self, selected_name: String) {
        if let Some(mut path) = self.file_path.clone() {
            // 更新文件名
            path.set_file_name(&selected_name);
            self.do_save(path);
        } else {
            // 新建文件，使用建议的文件名打开保存对话框
            if let Some(new_path) = rfd::FileDialog::new()
                .set_file_name(&selected_name)
                .save_file()
            {
                self.do_save(new_path);
            }
        }
        self.show_save_dialog = false;
        self.save_suggestions.clear();
        self.save_filename_input.clear();
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
    
    fn get_preview_mode(&self) -> PreviewMode {
        match &self.file_type {
            FileType::Markdown => PreviewMode::Markdown,
            FileType::SVG => PreviewMode::Image,
            FileType::Image(_) => PreviewMode::Image,
            FileType::Mermaid => PreviewMode::Diagram,
            FileType::Dot => PreviewMode::Diagram,
            FileType::PlantUML => PreviewMode::Diagram,
            FileType::Markmap => PreviewMode::Diagram,
            // 文档格式（阅读器模式）
            FileType::Word => PreviewMode::Reader,
            FileType::PDF => PreviewMode::Reader,
            FileType::Excel => PreviewMode::Reader,
            FileType::PowerPoint => PreviewMode::Reader,
            _ if self.file_type.to_syntax_name().is_some() => PreviewMode::Highlighted,
            _ => PreviewMode::Editor,
        }
    }
}

impl eframe::App for TextEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.is_dark_theme {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }
        
        // 处理快捷键
        let shortcuts = [
            (egui::Key::N, egui::Modifiers::CTRL, "新建"),
            (egui::Key::O, egui::Modifiers::CTRL, "打开"),
            (egui::Key::S, egui::Modifiers::CTRL, "保存"),
            (egui::Key::F4, egui::Modifiers::ALT, "退出"),
        ];
        
        for (key, mods, action) in shortcuts {
            if ctx.input_mut(|i| i.consume_key(mods, key)) {
                match action {
                    "新建" => {
                        self.text.clear();
                        self.file_path = None;
                        self.file_type = FileType::PlainText;
                    },
                    "打开" => {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.open_file(path);
                        }
                    },
                    "保存" => {
                        self.save_file();
                    },
                    _ => {}
                }
            }
        }
        
        // 处理拖拽文件
        preview::handle_dropped_files(ctx, |paths| {
            if let Some(path) = paths.first() {
                self.open_file(path.clone());
            }
        });
        
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("文件", |ui| {
                    if ui.button("新建 (Ctrl+N)").clicked() {
                        self.text.clear();
                        self.file_path = None;
                        self.file_type = FileType::PlainText;
                        ui.close_menu();
                    }
                    
                    if ui.button("打开... (Ctrl+O)").clicked() {
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
                    
                    if ui.button("保存 (Ctrl+S)").clicked() {
                        self.save_file();
                        ui.close_menu();
                    }
                    
                    if ui.button("另存为...").clicked() {
                        let suggested = suggest_filename(&self.text, self.file_path.as_deref(), &self.file_type);
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
                    if ui.button("关联所有文件类型").clicked() {
                        match file_association::register_all_extensions() {
                            Ok(_) => self.message = Some("✓ 所有文件类型已关联，右键菜单已添加".to_string()),
                            Err(e) => self.message = Some(format!("✗ 关联失败: {}", e)),
                        }
                        ui.close_menu();
                    }

                    if ui.button("取消所有文件关联").clicked() {
                        match file_association::unregister_all_extensions() {
                            Ok(_) => self.message = Some("✓ 所有文件关联已取消，右键菜单已移除".to_string()),
                            Err(e) => self.message = Some(format!("✗ 取消关联失败: {}", e)),
                        }
                        ui.close_menu();
                    }

                    ui.separator();

                    if ui.button("添加右键菜单").clicked() {
                        match file_association::register_context_menu() {
                            Ok(_) => self.message = Some("✓ 右键菜单已添加".to_string()),
                            Err(e) => self.message = Some(format!("✗ 添加失败: {}", e)),
                        }
                        ui.close_menu();
                    }

                    if ui.button("移除右键菜单").clicked() {
                        match file_association::unregister_context_menu() {
                            Ok(_) => self.message = Some("✓ 右键菜单已移除".to_string()),
                            Err(e) => self.message = Some(format!("✗ 移除失败: {}", e)),
                        }
                        ui.close_menu();
                    }

                    ui.separator();

                    if ui.button("查看关联状态").clicked() {
                        let registered = file_association::get_registered_extensions();
                        let unregistered = file_association::get_unregistered_extensions();
                        let total = registered.len() + unregistered.len();
                        self.message = Some(format!(
                            "已关联: {}/{} 类型",
                            registered.len(),
                            total
                        ));
                        ui.close_menu();
                    }
                    
                    if ui.button("📁 已支持文件类型").clicked() {
                        self.current_tab = Tab::FileTypes;
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("帮助", |ui| {
                    if ui.button("关于 IrisNote").clicked() {
                        ui.close_menu();
                    }
                    
                    ui.separator();
                    
                    ui.menu_button("📁 支持的文件类型", |ui| {
                        let categories = file_type::get_file_type_categories();
                        
                        ui.label("支持的文件类型分类：");
                        ui.add_space(5.0);
                        
                        for category in categories {
                            ui.horizontal(|ui| {
                                ui.label(&category.name);
                                ui.add_space(5.0);
                                let exts: Vec<String> = category.extensions.iter()
                                    .map(|e| e.clone())
                                    .collect();
                                ui.label(RichText::new(exts.join(" ")).size(11.0).color(Color32::GRAY));
                            });
                        }
                        
                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(5.0);
                        ui.label("共计 50+ 文件类型");
                    });
                    
                    ui.separator();
                    ui.label("IrisNote v0.1.0");
                    ui.label("智能文本编辑器");
                    ui.label("支持 50+ 文件类型");
                    ui.label("语法高亮 | 预览 | 智能分析");
                    
                    ui.separator();
                    ui.hyperlink_to("GitHub", "https://github.com/itszzl-sudo/irisnote");
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let theme_text = if self.is_dark_theme { "☀" } else { "🌙" };
                    if ui.small_button(theme_text).clicked() {
                        self.is_dark_theme = !self.is_dark_theme;
                    }
                });
            });
        });
        
        // 保存命名建议对话框
        if self.show_save_dialog {
            let mut selected_name: Option<String> = None;
            let mut should_close = false;
            let mut should_save = false;
            
            egui::Window::new("保存文件")
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label(RichText::new("文件名:").size(14.0));
                    ui.add_space(5.0);
                    
                    // 文件名输入框
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.save_filename_input)
                            .desired_width(250.0)
                    );
                    
                    // 回车保存
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        if !self.save_filename_input.is_empty() {
                            should_save = true;
                        }
                    }
                    
                    ui.add_space(10.0);
                    
                    // 建议名称
                    ui.label(RichText::new("建议名称:").size(12.0).color(Color32::from_rgb(150, 150, 150)));
                    ui.add_space(3.0);
                    
                    for (i, suggestion) in self.save_suggestions.iter().enumerate() {
                        if ui.small_button(RichText::new(format!("{}. {}", i + 1, suggestion)).size(12.0)).clicked() {
                            selected_name = Some(suggestion.clone());
                        }
                    }
                    
                    ui.add_space(15.0);
                    
                    if ui.button("取消").clicked() {
                        should_close = true;
                    }
                });
            
            if should_save {
                self.handle_save_with_suggestion(self.save_filename_input.clone());
            } else if let Some(name) = selected_name {
                self.handle_save_with_suggestion(name);
            } else if should_close {
                self.show_save_dialog = false;
                self.save_suggestions.clear();
                self.save_filename_input.clear();
            }
        }
        
        // 消息超时处理
        if let Some(timeout) = self.message_timeout {
            if std::time::Instant::now() >= timeout {
                self.message = None;
                self.message_timeout = None;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, Tab::Editor, "编辑器");
                if self.file_type.has_visual_preview() {
                    ui.selectable_value(&mut self.current_tab, Tab::Split, "分屏");
                    ui.selectable_value(&mut self.current_tab, Tab::Preview, "预览");
                }
            });
            ui.separator();
            
            match self.current_tab {
                Tab::Editor => {
                    self.render_editor(ui);
                }
                Tab::Preview => {
                    let preview_mode = self.get_preview_mode();
                    render_preview(ui, &self.text, &self.file_type, &preview_mode);
                }
                Tab::FileTypes => {
                    // 已支持文件类型展示页面
                    ui.horizontal(|ui| {
                        let response = ui.text_edit_singleline(&mut self.file_type_search_query);
                        if response.changed() {
                            self.file_type_search_results = file_type::search_file_types(&self.file_type_search_query);
                        }
                        if ui.button("搜索").clicked() {
                            self.file_type_search_results = file_type::search_file_types(&self.file_type_search_query);
                        }
                        ui.add_space(10.0);
                        if ui.button("✕ 关闭").clicked() {
                            self.current_tab = Tab::Editor;
                        }
                    });
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);
                    
                    // 平铺展示文件类型分类
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        if self.file_type_search_query.is_empty() {
                            // 显示分类视图
                            let categories = file_type::get_file_type_categories();
                            for category in categories {
                                ui.group(|ui| {
                                    ui.heading(&category.name);
                                    ui.add_space(5.0);
                                    ui.columns(3, |columns| {
                                        for (i, ext) in category.extensions.iter().enumerate() {
                                            columns[i % 3].label(
                                                RichText::new(ext)
                                                    .font(FontId::monospace(13.0))
                                                    .color(Color32::from_rgb(100, 180, 255))
                                            );
                                        }
                                    });
                                });
                                ui.add_space(10.0);
                            }
                        } else {
                            // 显示搜索结果
                            let count = self.file_type_search_results.len();
                            ui.label(format!("找到 {} 个匹配结果", count));
                            ui.add_space(10.0);
                            
                            if self.file_type_search_results.is_empty() {
                                ui.label("未找到匹配的文件类型");
                            } else {
                                ui.columns(2, |columns| {
                                    for (ext, desc) in &self.file_type_search_results {
                                        columns[0].label(
                                            RichText::new(ext)
                                                .font(FontId::monospace(13.0))
                                                .color(Color32::from_rgb(100, 180, 255))
                                        );
                                        columns[1].label(desc);
                                    }
                                });
                            }
                        }
                    });
                }
                Tab::Split => {
                    // 分屏模式：左边预览，右边编辑
                    let available_size = ui.available_size();
                    let min_editor_width = (available_size.x * 0.1).max(50.0); // 源码区域至少10%宽度或50像素
                    let preview_width = available_size.x - min_editor_width - 3.0; // 减去分隔线宽度
                    
                    ui.horizontal(|ui| {
                        // 设置左侧区域大小
                        ui.allocate_ui(
                            [preview_width, available_size.y].into(),
                            |ui| {
                                ui.set_max_width(preview_width);
                                ui.heading("预览");
                                ui.separator();
                                let preview_mode = self.get_preview_mode();
                                
                                egui::ScrollArea::vertical()
                                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                                    .show(ui, |ui| {
                                        let text_width = ui.available_size().x;
                                        ui.allocate_ui_with_layout(
                                            [text_width, f32::INFINITY].into(),
                                            egui::Layout::top_down_justified(egui::Align::Min),
                                            |ui| {
                                                render_preview(ui, &self.text, &self.file_type, &preview_mode);
                                            },
                                        );
                                    });
                            },
                        );
                        
                        // 分隔线
                        ui.add(egui::Separator::default().spacing(3.0));
                        
                        // 设置右侧区域大小
                        ui.allocate_ui(
                            [min_editor_width, available_size.y].into(),
                            |ui| {
                                ui.set_max_width(min_editor_width);
                                ui.heading("编辑器");
                                ui.separator();
                                self.render_editor(ui);
                            },
                        );
                    });
                }
            }
        });
        
        // 状态栏
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // 左侧显示消息或状态
                if let Some(ref msg) = self.message {
                    ui.label(RichText::new(msg).color(Color32::GREEN));
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 右侧显示光标位置和文件信息
                    let (line, col) = self.get_cursor_position();
                    let line_count = self.text.lines().count();
                    let char_count = self.text.chars().count();
                    
                    let file_info = if let Some(ref path) = self.file_path {
                        format!("{} | 第 {} 行, 第 {} 列 | {} 行 | {} 字符", 
                            path.display(), line, col, line_count, char_count)
                    } else {
                        format!("第 {} 行, 第 {} 列 | {} 行 | {} 字符", 
                            line, col, line_count, char_count)
                    };
                    
                    ui.label(RichText::new(file_info).size(12.0).color(Color32::from_rgb(150, 150, 150)));
                    
                    ui.separator();
                    
                    // 显示文件类型
                    let type_name = format!("{}", self.file_type);
                    ui.label(RichText::new(type_name).size(12.0).color(Color32::from_rgb(100, 180, 255)));
                });
            });
        });
    }
}

impl TextEditor {
    fn render_editor(&mut self, ui: &mut egui::Ui) {
        let syntax_name = self.file_type.to_syntax_name();
        let has_highlighting = syntax_name.is_some();
        
        egui::ScrollArea::vertical()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
            .show(ui, |ui| {
                if has_highlighting {
                    let theme = if self.is_dark_theme {
                        CODE_THEME_DARK.get_or_init(|| CodeTheme::dark())
                    } else {
                        CODE_THEME_LIGHT.get_or_init(|| CodeTheme::light())
                    };

                    let language = syntax_name.unwrap_or("text");

                    let mut text = self.text.clone();
                    let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                        let mut layout_job = highlight(ui.ctx(), theme, string, language);
                        layout_job.wrap.max_width = wrap_width;
                        ui.fonts(|f| f.layout_job(layout_job))
                    };

                    egui::TextEdit::multiline(&mut text)
                        .font(FontId::monospace(14.0))
                        .code_editor()
                        .layouter(&mut layouter)
                        .hint_text("在此输入文本...")
                        .desired_width(ui.available_width())
                        .show(ui);

                    if text != self.text {
                        self.text = text;
                        self.update_file_type();
                        // 用户修改文件，清除保存消息
                        self.message = None;
                        self.message_timeout = None;
                    }
                } else {
                    let mut text = self.text.clone();
                    
                    egui::TextEdit::multiline(&mut text)
                        .font(FontId::monospace(14.0))
                        .hint_text("在此输入文本...")
                        .desired_width(ui.available_width())
                        .show(ui);
                    
                    if text != self.text {
                        self.text = text;
                        self.update_file_type();
                        // 用户修改文件，清除保存消息
                        self.message = None;
                        self.message_timeout = None;
                    }
                }
            });
    }
}
