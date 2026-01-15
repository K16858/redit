use crossterm::style::Color;
use std::sync::Mutex;

use crate::editor::annotated_string::AnnotationType;
use crate::editor::highlight::config_file::{ColorRgb, ColorsConfig, load_colors_config};

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

static COLOR_SCHEME: std::sync::LazyLock<Mutex<ColorScheme>> = std::sync::LazyLock::new(|| {
    let default = default_color_scheme();
    let merged_scheme = if let Ok(config_file) = load_colors_config(None) {
        merge_color_scheme(&default, config_file.colors.as_ref())
    } else {
        default
    };
    Mutex::new(merged_scheme)
});

fn default_color_scheme() -> ColorScheme {
    #[cfg(debug_assertions)]
    {
        use std::path::Path;
        if let Ok(config_file) =
            load_colors_config(Some(Path::new("docs/examples/default/colors.toml")))
            && let Some(colors_config) = config_file.colors {
                return merge_color_scheme(&DEFAULT_COLOR_SCHEME, Some(&colors_config));
            }
    }

    // Release build or fallback: use hardcoded values
    DEFAULT_COLOR_SCHEME
}

pub struct Attribute {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

impl From<AnnotationType> for Attribute {
    fn from(annotation_type: AnnotationType) -> Self {
        let scheme = COLOR_SCHEME.lock().unwrap();
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

fn color_rgb_to_color(rgb: &ColorRgb) -> Color {
    Color::Rgb {
        r: rgb.r,
        g: rgb.g,
        b: rgb.b,
    }
}

pub fn merge_color_scheme(
    default: &ColorScheme,
    file_config: Option<&ColorsConfig>,
) -> ColorScheme {
    let Some(file_config) = file_config else {
        return ColorScheme {
            match_fg: default.match_fg,
            match_bg: default.match_bg,
            selected_match_fg: default.selected_match_fg,
            selected_match_bg: default.selected_match_bg,
            keyword: default.keyword,
            number: default.number,
            type_name: default.type_name,
            primitive_type: default.primitive_type,
            string: default.string,
            comment: default.comment,
            brackets: default.brackets,
        };
    };

    let brackets = if let Some(file_brackets) = &file_config.brackets {
        let mut result = default.brackets;
        for (i, bracket_color) in file_brackets.iter().enumerate() {
            if i < result.len() {
                result[i] = color_rgb_to_color(bracket_color);
            }
        }
        result
    } else {
        default.brackets
    };

    ColorScheme {
        match_fg: default.match_fg,
        match_bg: default.match_bg,
        selected_match_fg: default.selected_match_fg,
        selected_match_bg: default.selected_match_bg,
        keyword: file_config
            .keyword
            .as_ref()
            .map_or(default.keyword, color_rgb_to_color),
        number: file_config
            .number
            .as_ref()
            .map_or(default.number, color_rgb_to_color),
        type_name: file_config
            .type_name
            .as_ref()
            .map_or(default.type_name, color_rgb_to_color),
        primitive_type: file_config
            .primitive_type
            .as_ref()
            .map_or(default.primitive_type, color_rgb_to_color),
        string: file_config
            .string
            .as_ref()
            .map_or(default.string, color_rgb_to_color),
        comment: file_config
            .comment
            .as_ref()
            .map_or(default.comment, color_rgb_to_color),
        brackets,
    }
}
