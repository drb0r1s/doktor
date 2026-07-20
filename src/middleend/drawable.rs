use crate::frontend::resolved_ast::{SystemAttributes, SystemStyles};

use crate::middleend::layout::{LayoutProperties, Size, Location};

#[derive(Debug, Clone, PartialEq)]
pub struct DrawableBlockNode {
    pub block_type: String,
    pub tag: String,

    pub system_attributes: SystemAttributes,
    pub arbitrary_attributes: Vec<(String, String)>,
    pub system_styles: SystemStyles,
    pub arbitrary_styles: Vec<(String, String)>,

    pub layout_properties: LayoutProperties,
    pub size: Size,
    pub location: Location,

    pub children: Vec<DrawableBlockNode>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DrawableDoktorNode {
    pub children: Vec<DrawableBlockNode>,
}