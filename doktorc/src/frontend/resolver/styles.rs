use std::collections::HashMap;

use crate::frontend::parser_ast::{Style, ParserBlockNode};
use crate::frontend::resolver::ast::invalids::ResolverWarning;

pub struct Styles {
    pub warnings: Vec<ResolverWarning>,
}

impl Styles {
    pub fn new() -> Self {
        Styles {
            warnings: Vec::new(),
        }
    }
    
    pub fn collect_tag_styles(children: &[ParserBlockNode]) -> HashMap<String, Vec<Style>> {
        let mut tag_styles: HashMap<String, Vec<Style>> = HashMap::new();

        for block in children {
            if block.block_type == "Styles" {
                for style_block in &block.children {
                    if style_block.block_type == "Style" && !style_block.tag.is_empty() {
                        tag_styles.insert(style_block.tag.clone(), style_block.styles.clone());
                    }
                }
            }
        }

        tag_styles
    }

    pub fn filter_style_blocks(&mut self, children: Vec<ParserBlockNode>) -> Vec<ParserBlockNode> {
        children.into_iter().filter_map(|child| {
            match child.block_type.as_str() {
                "Styles" => {
                    self.warnings.push(ResolverWarning {
                        message: "\"Styles\" block is only valid at the top level of the document, it is ignored otherwise".to_string(),
                        line: child.line,
                        column: child.column,
                    });

                    None
                }

                "Style" => {
                    self.warnings.push(ResolverWarning {
                        message: "\"Style\" block is only valid as a child of a \"Styles\" block at the top level of the document, it is ignored otherwise".to_string(),
                        line: child.line,
                        column: child.column,
                    });

                    None
                }

                _ => Some(child)
            }
        }).collect()
    }
}