use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::frontend::parser_ast::ParserBlockNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParamType {
    Text,
    Number,
    Bool,
    Color,
}

pub fn parse_param_type(value: &str) -> Option<ParamType> {
    match value {
        "text" => Some(ParamType::Text),
        "number" => Some(ParamType::Number),
        "bool" => Some(ParamType::Bool),
        "color" => Some(ParamType::Color),
        _ => None,
    }
}

pub struct Collection {
    pub body: ParserBlockNode,
    pub attributes: HashMap<String, ParamType>,
    pub styles: HashMap<String, ParamType>,
}

pub type CollectionMap = HashMap<String, Collection>;