use crate::frontend::resolver_ast::{SystemAttributes, SystemStyles};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}

#[derive(serde::Deserialize)]
pub struct TextMeasurement {
    pub path: Vec<usize>,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShaperBlockNode {
    pub block_type: String,
    pub tag: String,

    pub system_attributes: SystemAttributes,
    pub arbitrary_attributes: Vec<(String, String)>,
    pub system_styles: SystemStyles,
    pub arbitrary_styles: Vec<(String, String)>,

    pub size: Size,
    pub location: Location,

    pub children: Vec<ShaperBlockNode>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShaperDoktorNode {
    pub children: Vec<ShaperBlockNode>,
}