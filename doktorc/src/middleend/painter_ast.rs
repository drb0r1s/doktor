use crate::frontend::resolver_ast::{RGB, Font};

use crate::middleend::shaper_ast::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub location: Location,
    pub width: f32,
    pub height: f32,
    pub color: RGB,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub location: Location,
    pub content: String,
    pub font_size: f32,
    pub font_family: Font,
    pub color: RGB,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub location: Location,
    pub width: f32,
    pub height: f32,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DrawStructure {
    Rectangle(Rectangle),
    Text(Text),
    Image(Image),
}