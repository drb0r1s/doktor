use serde::{Serialize, Deserialize};

use crate::middleend::layout::{Layout, Direction, Alignment};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SystemStyles {
    pub layout: Option<Layout>, // simple, free
    pub direction: Option<Direction>, // horizontal, vertical
    pub alignment: Option<Alignment>, // start, center, end
    pub alignment_x: Option<Alignment>,
    pub alignment_y: Option<Alignment>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub position_x: Option<f32>,
    pub position_y: Option<f32>,
    pub content_color: Option<RGB>,
    pub background_color: Option<RGB>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedBlockNode {
    pub block_type: String,
    pub tag: String,
    pub system_attributes: SystemAttributes,
    pub arbitrary_attributes: Vec<(String, String)>,
    pub system_styles: SystemStyles,
    pub arbitrary_styles: Vec<(String, String)>,
    pub children: Vec<ResolvedBlockNode>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedDoktorNode {
    pub children: Vec<ResolvedBlockNode>,
}