use serde::{Serialize, Deserialize};

use crate::frontend::resolver::ast::system_attributes::SystemAttributes;
use crate::frontend::resolver::ast::system_styles::SystemStyles;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolverBlockNode {
    pub id: u32,
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