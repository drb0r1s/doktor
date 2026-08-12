use crate::frontend::resolver::ast::system_styles::BorderType;

use crate::middleend::shaper_ast::{Location, Clip};

use crate::collections::rgb::RGB;
use crate::collections::font::Font;

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub id: u32,
    pub location: Location,
    pub width: f32,
    pub height: f32,
    pub is_scrollable_x: bool,
    pub is_scrollable_y: bool,
    pub scrollable_width: f32,
    pub scrollable_height: f32,
    pub scroll_offset: Location,
    pub clip: Clip,
    pub background_color: RGB,
    pub border_top_color: RGB,
    pub border_bottom_color: RGB,
    pub border_left_color: RGB,
    pub border_right_color: RGB,
    pub border_top_size: f32,
    pub border_bottom_size: f32,
    pub border_left_size: f32,
    pub border_right_size: f32,
    pub border_top_type: BorderType,
    pub border_bottom_type: BorderType,
    pub border_left_type: BorderType,
    pub border_right_type: BorderType,
    pub opacity: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub id: u32,
    pub location: Location,
    pub clip: Clip,
    pub content: String,
    pub content_color: RGB,
    pub content_size: f32,
    pub content_font: Font,
    pub background_color: RGB,
    pub border_top_color: RGB,
    pub border_bottom_color: RGB,
    pub border_left_color: RGB,
    pub border_right_color: RGB,
    pub border_top_size: f32,
    pub border_bottom_size: f32,
    pub border_left_size: f32,
    pub border_right_size: f32,
    pub border_top_type: BorderType,
    pub border_bottom_type: BorderType,
    pub border_left_type: BorderType,
    pub border_right_type: BorderType,
    pub opacity: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub id: u32,
    pub location: Location,
    pub width: f32,
    pub height: f32,
    pub clip: Clip,
    pub source: String,
    pub background_color: RGB,
    pub border_top_color: RGB,
    pub border_bottom_color: RGB,
    pub border_left_color: RGB,
    pub border_right_color: RGB,
    pub border_top_size: f32,
    pub border_bottom_size: f32,
    pub border_left_size: f32,
    pub border_right_size: f32,
    pub border_top_type: BorderType,
    pub border_bottom_type: BorderType,
    pub border_left_type: BorderType,
    pub border_right_type: BorderType,
    pub opacity: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DrawStructure {
    Rectangle(Rectangle),
    Text(Text),
    Image(Image),
}