use crossterm::style::Color;

use crate::editor::annotated_string::AnnotationType;

pub struct ColorScheme {
    pub match_fg: Color,
    pub match_bg: Color,
    pub selected_match_fg: Color,
    pub selected_match_bg: Color,
    pub keyword: Color,
    pub number: Color,
    pub type_name: Color,
    pub primitive_type: Color,
    pub string: Color,
    pub comment: Color,
    pub brackets: [Color; 4],
}

pub const DEFAULT_COLOR_SCHEME: ColorScheme = ColorScheme {
    match_fg: Color::Rgb {
        r: 255,
        g: 255,
        b: 255,
    },
    match_bg: Color::Rgb {
        r: 100,
        g: 100,
        b: 100,
    },
    selected_match_fg: Color::Rgb {
        r: 255,
        g: 255,
        b: 255,
    },
    selected_match_bg: Color::Rgb {
        r: 200,
        g: 180,
        b: 60,
    },
    keyword: Color::Rgb {
        r: 86,
        g: 156,
        b: 214,
    },
    number: Color::Rgb {
        r: 181,
        g: 206,
        b: 168,
    },
    type_name: Color::Rgb {
        r: 78,
        g: 201,
        b: 176,
    },
    primitive_type: Color::Rgb {
        r: 78,
        g: 201,
        b: 176,
    },
    string: Color::Rgb {
        r: 206,
        g: 145,
        b: 120,
    },
    comment: Color::Rgb {
        r: 106,
        g: 153,
        b: 85,
    },
    brackets: [
        Color::Rgb {
            r: 140,
            g: 140,
            b: 140,
        },
        Color::Rgb {
            r: 97,
            g: 175,
            b: 239,
        },
        Color::Rgb {
            r: 198,
            g: 120,
            b: 221,
        },
        Color::Rgb {
            r: 152,
            g: 195,
            b: 121,
        },
    ],
};

pub struct Attribute {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

impl From<AnnotationType> for Attribute {
    fn from(annotation_type: AnnotationType) -> Self {
        let scheme = &DEFAULT_COLOR_SCHEME;
        match annotation_type {
            AnnotationType::Match => Self {
                foreground: Some(scheme.match_fg),
                background: Some(scheme.match_bg),
            },
            AnnotationType::SelectedMatch => Self {
                foreground: Some(scheme.selected_match_fg),
                background: Some(scheme.selected_match_bg),
            },
            AnnotationType::Keyword => Self {
                foreground: Some(scheme.keyword),
                background: None,
            },
            AnnotationType::Number => Self {
                foreground: Some(scheme.number),
                background: None,
            },
            AnnotationType::Type => Self {
                foreground: Some(scheme.type_name),
                background: None,
            },
            AnnotationType::PrimitiveType => Self {
                foreground: Some(scheme.primitive_type),
                background: None,
            },
            AnnotationType::String => Self {
                foreground: Some(scheme.string),
                background: None,
            },
            AnnotationType::Comment => Self {
                foreground: Some(scheme.comment),
                background: None,
            },
            AnnotationType::Bracket0 => Self {
                foreground: Some(scheme.brackets[0]),
                background: None,
            },
            AnnotationType::Bracket1 => Self {
                foreground: Some(scheme.brackets[1]),
                background: None,
            },
            AnnotationType::Bracket2 => Self {
                foreground: Some(scheme.brackets[2]),
                background: None,
            },
            AnnotationType::Bracket3 => Self {
                foreground: Some(scheme.brackets[3]),
                background: None,
            },
        }
    }
}
