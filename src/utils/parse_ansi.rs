use ansitok::{AnsiColor, ElementKind, Output, VisualAttribute};
use gpui::{Div, ParentElement as _, Rgba, Styled as _};

use crate::theme::Theme;

#[derive(Debug, Clone)]
pub struct Log {
    pub text: String,
    pub color: Option<Rgba>,
    pub bold: bool,
}

impl From<Log> for Div {
    fn from(value: Log) -> Self {
        let mut div = gpui::div().child(value.text);

        if let Some(color) = value.color {
            div = div.text_color(color);
        }
        if value.bold {
            div = div.font_weight(gpui::FontWeight::BOLD);
        }

        div
    }
}

pub fn parse_ansi(input: &str, theme: &Theme) -> Vec<Log> {
    let mut spans = Vec::new();
    let mut current_color = None;
    let mut current_bold = false;

    for element in ansitok::parse_ansi(input) {
        match element.kind() {
            ElementKind::Text => {
                let text = &input[element.start()..element.end()];

                if !text.is_empty() {
                    spans.push(Log {
                        text: text.to_string(),
                        color: current_color,
                        bold: current_bold,
                    });
                }
            }
            ElementKind::Sgr => {
                let sgr_slice = &input[element.start()..element.end()];

                for output in ansitok::parse_ansi_sgr(sgr_slice) {
                    let Output::Escape(attribute) = output else {
                        continue;
                    };

                    match attribute {
                        VisualAttribute::Reset(_) => {
                            current_color = None;
                            current_bold = false;
                        }
                        VisualAttribute::Bold => {
                            current_bold = true;
                        }
                        VisualAttribute::FgColor(AnsiColor::Bit4(code)) => {
                            current_color = map_ansi_to_color(code, theme);
                        }

                        _ => (),
                    }
                }
            }

            _ => (),
        }
    }

    if spans.is_empty() {
        spans.push(Log {
            text: input.to_string(),
            color: None,
            bold: false,
        });
    }

    spans
}

fn map_ansi_to_color(code: u8, theme: &Theme) -> Option<Rgba> {
    let index = match code {
        0..=15 => code,
        30..=37 => code - 30,
        40..=47 => code - 40,
        90..=97 => code - 90 + 8,
        100..=107 => code - 100 + 8,
        _ => return None,
    };

    match index {
        0 => Some(theme.surface_strong),
        1 | 9 => Some(theme.error),
        2 | 10 => Some(theme.success),
        3 | 11 => Some(theme.warning),
        4 | 12 => Some(theme.info),
        5 | 13 => Some(theme.accent_alt),
        6 | 14 => Some(theme.accent),
        7 => Some(theme.text),
        8 => Some(theme.text_muted),
        15 => Some(theme.text_strong),

        _ => None,
    }
}
