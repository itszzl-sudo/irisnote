use std::sync::OnceLock;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Highlighter, HighlightIterator, Theme};
use syntect::easy::HighlightLines;
use syntect::util::LinesWithEndings;

static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

pub struct SyntaxHighlighter {
    syntax_set: &'static SyntaxSet,
    theme_set: &'static ThemeSet,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        let syntax_set = SYNTAX_SET.get_or_init(|| {
            SyntaxSet::load_defaults_newlines()
        });
        
        let theme_set = THEME_SET.get_or_init(|| {
            ThemeSet::load_defaults()
        });
        
        Self {
            syntax_set,
            theme_set,
        }
    }
    
    pub fn highlight(&self, text: &str, syntax_name: &str) -> Vec<HighlightedLine> {
        let syntax = self.syntax_set.find_syntax_by_name(syntax_name)
            .or_else(|| self.syntax_set.find_syntax_by_extension(syntax_name))
            .or_else(|| self.syntax_set.find_syntax_by_first_line(text))
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
        
        let theme = &self.theme_set.themes["base16-ocean.dark"];
        let mut h = HighlightLines::new(syntax, theme);
        
        let mut lines = Vec::new();
        for line in LinesWithEndings::from(text) {
            let ranges: Vec<(syntect::highlighting::Style, &str)> = h.highlight_line(line, self.syntax_set).unwrap_or_default();
            
            let segments: Vec<HighlightSegment> = ranges
                .into_iter()
                .map(|(style, text)| HighlightSegment {
                    text: text.to_string(),
                    color: style_to_color(style),
                })
                .collect();
            
            lines.push(HighlightedLine { segments });
        }
        
        lines
    }
    
    pub fn get_available_syntaxes(&self) -> Vec<String> {
        self.syntax_set.syntaxes()
            .iter()
            .map(|s| s.name.clone())
            .collect()
    }
}

fn style_to_color(style: syntect::highlighting::Style) -> egui::Color32 {
    let fg = style.foreground;
    egui::Color32::from_rgba_unmultiplied(fg.r, fg.g, fg.b, 255)
}

#[derive(Debug, Clone)]
pub struct HighlightedLine {
    pub segments: Vec<HighlightSegment>,
}

#[derive(Debug, Clone)]
pub struct HighlightSegment {
    pub text: String,
    pub color: egui::Color32,
}
