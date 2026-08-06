use serde::{Serialize, Deserialize};

use crate::frontend::resolver_ast::{SystemAttributes, SystemStyles};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Clip {
    pub x: (f32, f32),
    pub y: (f32, f32),
}

#[derive(Deserialize)]
pub struct TextMeasurement {
    pub path: Vec<usize>,
    pub width: f32,
    pub height: f32,
}

#[derive(Deserialize)]
pub struct ImageMeasurement {
    pub path: Vec<usize>,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ShaperBlockNode {
    pub id: u32,

    pub block_type: String,
    pub tag: String,

    pub system_attributes: SystemAttributes,
    pub arbitrary_attributes: Vec<(String, String)>,
    pub system_styles: SystemStyles,
    pub arbitrary_styles: Vec<(String, String)>,

    pub size: Size,
    pub location: Location,
    pub clip: Clip,

    pub children: Vec<ShaperBlockNode>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShaperDoktorNode {
    pub children: Vec<ShaperBlockNode>,
}