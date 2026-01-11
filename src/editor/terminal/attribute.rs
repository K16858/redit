use crossterm::style::Color;

use crate::editor::annotated_string::AnnotationType;

pub struct Attribute {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

impl From<AnnotationType> for Attribute {
    fn from(annotation_type: AnnotationType) -> Self {
        match annotation_type {
            AnnotationType::Match => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(Color::Rgb {
                    r: 100,
                    g: 100,
                    b: 100,
                }),
            },
            AnnotationType::SelectedMatch => Self {
                foreground: Some(Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(Color::Rgb {
                    r: 200,
                    g: 180,
                    b: 60,
                }),
            },
            AnnotationType::Keyword => Self {
                foreground: Some(Color::Rgb {
                    r: 86,
                    g: 156,
                    b: 214,
                }),
                background: None,
            },
            AnnotationType::Number => Self {
                foreground: Some(Color::Rgb {
                    r: 206,
                    g: 145,
                    b: 120,
                }),
                background: None,
            },
            AnnotationType::Type => Self {
                foreground: Some(Color::Rgb {
                    r: 78,
                    g: 201,
                    b: 176,
                }),
                background: None,
            },
            AnnotationType::String => Self {
                foreground: Some(Color::Rgb {
                    r: 206,
                    g: 145,
                    b: 120,
                }),
                background: None,
            },
            AnnotationType::Comment => Self {
                foreground: Some(Color::Rgb {
                    r: 106,
                    g: 153,
                    b: 85,
                }),
                background: None,
            },
        }
    }
}
