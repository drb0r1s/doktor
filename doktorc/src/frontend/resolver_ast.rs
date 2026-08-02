use serde::{Serialize, Deserialize};

use crate::data::default_values;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Layout {
    Simple,
    Free,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Font {
    Arial = 0,
    Helvetica = 1,
    Verdana = 2,
    Tahoma = 3,
    TrebuchetMS = 4,
    SegoeUI = 5,
    Georgia = 6,
    TimesNewRoman = 7,
    Garamond = 8,
    Baskerville = 9,
    CourierNew = 10,
    Consolas = 11,
    SansSerif = 12,
    Serif = 13,
    Monospace = 14,
}

const FONT_NAMES: &[(&str, Font)] = &[
    ("arial", Font::Arial),
    ("helvetica", Font::Helvetica),
    ("verdana", Font::Verdana),
    ("tahoma", Font::Tahoma),
    ("trebuchet_ms", Font::TrebuchetMS),
    ("segoe_ui", Font::SegoeUI),
    ("georgia", Font::Georgia),
    ("times_new_roman", Font::TimesNewRoman),
    ("garamond", Font::Garamond),
    ("baskerville", Font::Baskerville),
    ("courier_new", Font::CourierNew),
    ("consolas", Font::Consolas),
    ("sans_serif", Font::SansSerif),
    ("serif", Font::Serif),
    ("monospace", Font::Monospace),
];

pub fn parse_font(value: &str) -> Option<Font> {
    FONT_NAMES.iter().find(|(name, _)| *name == value).map(|(_, font)| *font)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BorderType {
    None = 0,
    Solid = 1,
    Dashed = 2,
    Dotted = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Overflow {
    False = 0,
    True = 1,
    Scroll = 2,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SystemAttributes {
    // Image
    pub source: Option<String>,
    // Text
    pub content: Option<String>,
    // Input
    pub placeholder: Option<String>,
    pub max_length: Option<u32>,
    pub min_length: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemStyles {
    pub layout: Layout, // simple, free
    pub direction: Direction, // horizontal, vertical
    pub alignment: Alignment, // start, center, end
    pub alignment_x: Option<Alignment>,
    pub alignment_y: Option<Alignment>,
    pub width: f32,
    pub height: f32,
    pub lock_dimensions: bool,
    pub lock_width: Option<bool>,
    pub lock_height: Option<bool>,
    pub position: f32,
    pub position_x: Option<f32>,
    pub position_y: Option<f32>,
    pub content_color: RGB,
    pub content_size: f32,
    pub content_font: Font,
    pub background_color: RGB,
    pub border_color: RGB,
    pub border_top_color: Option<RGB>,
    pub border_bottom_color: Option<RGB>,
    pub border_left_color: Option<RGB>,
    pub border_right_color: Option<RGB>,
    pub border_size: f32,
    pub border_top_size: Option<f32>,
    pub border_bottom_size: Option<f32>,
    pub border_left_size: Option<f32>,
    pub border_right_size: Option<f32>,
    pub border_type: BorderType,
    pub border_top_type: Option<BorderType>,
    pub border_bottom_type: Option<BorderType>,
    pub border_left_type: Option<BorderType>,
    pub border_right_type: Option<BorderType>,
    pub opacity: f32,
    pub spacing: f32,
    pub spacing_top: Option<f32>,
    pub spacing_bottom: Option<f32>,
    pub spacing_left: Option<f32>,
    pub spacing_right: Option<f32>,
    pub margin: Option<f32>,
    pub margin_top: Option<f32>,
    pub margin_bottom: Option<f32>,
    pub margin_left: Option<f32>,
    pub margin_right: Option<f32>,
    pub padding: Option<f32>,
    pub padding_top: Option<f32>,
    pub padding_bottom: Option<f32>,
    pub padding_left: Option<f32>,
    pub padding_right: Option<f32>,
    pub overflow: Overflow,
    pub overflow_x: Option<Overflow>,
    pub overflow_y: Option<Overflow>,
}

impl SystemStyles {
    pub fn default(is_text: bool) -> Self {
        SystemStyles {
            layout: default_values::DEFAULT_LAYOUT,
            direction: default_values::DEFAULT_DIRECTION,
            alignment: default_values::DEFAULT_ALIGNMENT,
            alignment_x: None,
            alignment_y: None,
            width: default_values::DEFAULT_WIDTH,
            height: default_values::DEFAULT_HEIGHT,
            position: default_values::DEFAULT_POSITION,
            lock_dimensions: default_values::DEFAULT_LOCK_DIMENSIONS,
            lock_width: None,
            lock_height: None,
            position_x: None,
            position_y: None,
            content_color: default_values::DEFAULT_CONTENT_COLOR,
            content_size: default_values::DEFAULT_CONTENT_SIZE,
            content_font: default_values::DEFAULT_CONTENT_FONT,
            background_color: if is_text { default_values::DEFAULT_TEXT_BACKGROUND_COLOR } else { default_values::DEFAULT_BACKGROUND_COLOR },
            border_color: default_values::DEFAULT_BORDER_COLOR,
            border_top_color: None,
            border_bottom_color: None,
            border_left_color: None,
            border_right_color: None,
            border_size: default_values::DEFAULT_BORDER_SIZE,
            border_top_size: None,
            border_bottom_size: None,
            border_left_size: None,
            border_right_size: None,
            border_type: default_values::DEFAULT_BORDER_TYPE,
            border_top_type: None,
            border_bottom_type: None,
            border_left_type: None,
            border_right_type: None,
            opacity: default_values::DEFAULT_OPACITY,
            spacing: default_values::DEFAULT_SPACING,
            spacing_top: None,
            spacing_bottom: None,
            spacing_left: None,
            spacing_right: None,
            margin: None,
            margin_top: None,
            margin_bottom: None,
            margin_left: None,
            margin_right: None,
            padding: None,
            padding_top: None,
            padding_bottom: None,
            padding_left: None,
            padding_right: None,
            overflow: default_values::DEFAULT_OVERFLOW,
            overflow_x: None,
            overflow_y: None,
        }
    }

    pub fn get_unambiguous_alignment(&self, alignment_type: &str) -> Alignment {
        match alignment_type {
            "x" => self.alignment_x.or(Some(self.alignment)).unwrap_or(default_values::DEFAULT_ALIGNMENT),
            "y" => self.alignment_y.or(Some(self.alignment)).unwrap_or(default_values::DEFAULT_ALIGNMENT),
            _ => default_values::DEFAULT_ALIGNMENT,
        }
    }

    pub fn get_unambiguous_lock_dimensions(&self, lock_dimensions_type: &str) -> bool {
        match lock_dimensions_type {
            "width" => self.lock_width.or(Some(self.lock_dimensions)).unwrap_or(default_values::DEFAULT_LOCK_DIMENSIONS),
            "height" => self.lock_height.or(Some(self.lock_dimensions)).unwrap_or(default_values::DEFAULT_LOCK_DIMENSIONS),
            _ => default_values::DEFAULT_LOCK_DIMENSIONS,
        }
    }

    pub fn get_unambiguous_position(&self, position_type: &str) -> f32 {
        match position_type {
            "x" => self.position_x.or(Some(self.position)).unwrap_or(default_values::DEFAULT_POSITION),
            "y" => self.position_y.or(Some(self.position)).unwrap_or(default_values::DEFAULT_POSITION),
            _ => default_values::DEFAULT_POSITION,
        }
    }

    pub fn get_unambiguous_border_color(&self, border_color_type: &str) -> RGB {
        match border_color_type {
            "top" => self.border_top_color.or(Some(self.border_color)).unwrap_or(default_values::DEFAULT_BORDER_COLOR),
            "bottom" => self.border_bottom_color.or(Some(self.border_color)).unwrap_or(default_values::DEFAULT_BORDER_COLOR),
            "left" => self.border_left_color.or(Some(self.border_color)).unwrap_or(default_values::DEFAULT_BORDER_COLOR),
            "right" => self.border_right_color.or(Some(self.border_color)).unwrap_or(default_values::DEFAULT_BORDER_COLOR),
            _ => default_values::DEFAULT_BORDER_COLOR,
        }
    }

    pub fn get_unambiguous_border_size(&self, border_size_type: &str) -> f32 {
        match border_size_type {
            "top" => self.border_top_size.or(Some(self.border_size)).unwrap_or(default_values::DEFAULT_BORDER_SIZE),
            "bottom" => self.border_bottom_size.or(Some(self.border_size)).unwrap_or(default_values::DEFAULT_BORDER_SIZE),
            "left" => self.border_left_size.or(Some(self.border_size)).unwrap_or(default_values::DEFAULT_BORDER_SIZE),
            "right" => self.border_right_size.or(Some(self.border_size)).unwrap_or(default_values::DEFAULT_BORDER_SIZE),
            _ => default_values::DEFAULT_BORDER_SIZE,
        }
    }

    pub fn get_unambiguous_border_type(&self, border_type_type: &str) -> BorderType {
        match border_type_type {
            "top" => self.border_top_type.or(Some(self.border_type)).unwrap_or(default_values::DEFAULT_BORDER_TYPE),
            "bottom" => self.border_bottom_type.or(Some(self.border_type)).unwrap_or(default_values::DEFAULT_BORDER_TYPE),
            "left" => self.border_left_type.or(Some(self.border_type)).unwrap_or(default_values::DEFAULT_BORDER_TYPE),
            "right" => self.border_right_type.or(Some(self.border_type)).unwrap_or(default_values::DEFAULT_BORDER_TYPE),
            _ => default_values::DEFAULT_BORDER_TYPE,
        }
    }

    pub fn get_unambiguous_spacing(&self, spacing_type: &str, orientation: &str) -> f32 {
        match spacing_type {
            "margin" => match orientation {
                "top" => self.margin_top.or(self.margin).or(self.spacing_top).or(Some(self.spacing)).unwrap_or(default_values::DEFAULT_SPACING),
                "bottom" => self.margin_bottom.or(self.margin).or(self.spacing_bottom).or(Some(self.spacing)).unwrap_or(default_values::DEFAULT_SPACING),
                "left" => self.margin_left.or(self.margin).or(self.spacing_left).or(Some(self.spacing)).unwrap_or(default_values::DEFAULT_SPACING),
                "right" => self.margin_right.or(self.margin).or(self.spacing_right).or(Some(self.spacing)).unwrap_or(default_values::DEFAULT_SPACING),
                _ => default_values::DEFAULT_SPACING,
            },

            "padding" => match orientation {
                "top" => self.padding_top.or(self.padding).or(self.spacing_top).or(Some(self.spacing)).unwrap_or(default_values::DEFAULT_SPACING),
                "bottom" => self.padding_bottom.or(self.padding).or(self.spacing_bottom).or(Some(self.spacing)).unwrap_or(default_values::DEFAULT_SPACING),
                "left" => self.padding_left.or(self.padding).or(self.spacing_left).or(Some(self.spacing)).unwrap_or(default_values::DEFAULT_SPACING),
                "right" => self.padding_right.or(self.padding).or(self.spacing_right).or(Some(self.spacing)).unwrap_or(default_values::DEFAULT_SPACING),
                _ => default_values::DEFAULT_SPACING,
            },
            _ => default_values::DEFAULT_SPACING,
        }
    }

    pub fn get_unambiguous_overflow(&self, overflow_type: &str) -> Overflow {
        match overflow_type {
            "x" => self.overflow_x.or(Some(self.overflow)).unwrap_or(default_values::DEFAULT_OVERFLOW),
            "y" => self.overflow_y.or(Some(self.overflow)).unwrap_or(default_values::DEFAULT_OVERFLOW),
            _ => default_values::DEFAULT_OVERFLOW,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolverBlockNode {
    pub block_type: String,
    pub tag: String,
    pub system_attributes: SystemAttributes,
    pub arbitrary_attributes: Vec<(String, String)>,
    pub system_styles: SystemStyles,
    pub arbitrary_styles: Vec<(String, String)>,
    pub children: Vec<ResolverBlockNode>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolverDoktorNode {
    pub children: Vec<ResolverBlockNode>,
}