use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::frontend::resolver::ast::system_attributes::SystemAttributes;
use crate::frontend::resolver::ast::system_styles::SystemStyles;

use crate::middleend::shaper_ast::{Size, Location, Clip};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollerBlockNode {
    pub id: Uuid,
    pub block_type: String,
    pub tag: String,
    pub system_attributes: SystemAttributes,
    pub arbitrary_attributes: Vec<(String, String)>,
    pub system_styles: SystemStyles,
    pub arbitrary_styles: Vec<(String, String)>,
    pub size: Size,
    pub location: Location,
    pub clip: Clip,
    pub is_scrollable_x: bool,
    pub is_scrollable_y: bool,
    pub scrollable_size: Size,
    pub scroll_offset: Location,
    pub children: Vec<ScrollerBlockNode>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollerDoktorNode {
    pub children: Vec<ScrollerBlockNode>,
}