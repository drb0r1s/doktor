use serde::{Serialize, Deserialize};

use crate::data::default_values;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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
    pub position: f32,
    pub position_x: Option<f32>,
    pub position_y: Option<f32>,
    pub content_color: RGB,
    pub content_size: f32,
    pub background_color: RGB,
}

impl SystemStyles {
    pub fn default() -> Self {
        SystemStyles {
            layout: default_values::DEFAULT_LAYOUT,
            direction: default_values::DEFAULT_DIRECTION,
            alignment: default_values::DEFAULT_ALIGNMENT,
            alignment_x: None,
            alignment_y: None,
            width: default_values::DEFAULT_WIDTH,
            height: default_values::DEFAULT_HEIGHT,
            position: default_values::DEFAULT_POSITION,
            position_x: None,
            position_y: None,
            content_color: default_values::DEFAULT_CONTENT_COLOR,
            content_size: default_values::DEFAULT_CONTENT_SIZE,
            background_color: default_values::DEFAULT_BACKGROUND_COLOR,
        }
    }

    pub fn get_unambiguous_alignment(&self, alignment_type: &str) -> Alignment {
        match alignment_type {
            "x" => self.alignment_x.or(Some(self.alignment)).unwrap_or(default_values::DEFAULT_ALIGNMENT),
            "y" => self.alignment_y.or(Some(self.alignment)).unwrap_or(default_values::DEFAULT_ALIGNMENT),
            _ => default_values::DEFAULT_ALIGNMENT,
        }
    }

    pub fn get_unambiguous_position(&self, position_type: &str) -> f32 {
        match position_type {
            "x" => self.position_x.or(Some(self.position)).unwrap_or(default_values::DEFAULT_POSITION),
            "y" => self.position_y.or(Some(self.position)).unwrap_or(default_values::DEFAULT_POSITION),
            _ => default_values::DEFAULT_POSITION,
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