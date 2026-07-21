use std::fmt;

use crate::frontend::ast::{Attribute, Style, BlockNode, DoktorNode};
use crate::frontend::resolved_ast::{RGB, SystemAttributes, SystemStyles, ResolvedBlockNode, ResolvedDoktorNode};

use crate::middleend::layout::{Layout, Direction, Alignment};

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticWarning {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for SemanticWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Semantic Warning at [{}:{}]: {}.",
            self.line, self.column, self.message
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Semantic Error at [{}:{}]: {}.",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for SemanticError {}

const SYSTEM_BLOCK_TYPES: &[&str] = &["Group", "Image", "Text", "Input", "Button", "Collection"];

pub struct Resolver {
    warnings: Vec<SemanticWarning>,
    errors: Vec<SemanticError>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn resolve(mut self, doktor_node: DoktorNode) -> (ResolvedDoktorNode, Vec<SemanticWarning>, Vec<SemanticError>) {
        let children = doktor_node.children.into_iter().map(|block_node| self.resolve_block(block_node)).collect();

        (ResolvedDoktorNode { children }, self.warnings, self.errors)
    }

    fn resolve_block(&mut self, block_node: BlockNode) -> ResolvedBlockNode {
        let resolved_block_type: &str = if SYSTEM_BLOCK_TYPES.contains(&block_node.block_type.as_str()) {
            &block_node.block_type
        } else {
            self.errors.push(SemanticError {
                message: format!(
                    "Unrecognized block type '{}', treating it as 'Group'",
                    block_node.block_type
                ),
                line: block_node.line,
                column: block_node.column,
            });

            "Group"
        };

        let (system_attributes, arbitrary_attributes) = self.resolve_attributes(resolved_block_type, block_node.attributes);
        let (system_styles, arbitrary_styles) = self.resolve_styles(block_node.styles);

        let children = block_node.children.into_iter().map(|child_node| self.resolve_block(child_node)).collect();

        ResolvedBlockNode {
            block_type: block_node.block_type,
            tag: block_node.tag,
            system_attributes,
            arbitrary_attributes,
            system_styles,
            arbitrary_styles,
            children,
            line: block_node.line,
            column: block_node.column,
        }
    }

    fn resolve_attributes(&mut self, block_type: &str, attributes: Vec<Attribute>) -> (SystemAttributes, Vec<(String, String)>) {
        let mut system_attributes = SystemAttributes::default();
        let mut arbitrary_attributes = Vec::new();

        for attribute in attributes {
            let recognized: bool = match (block_type, attribute.name.as_str()) {
                ("Image", "source") => {
                    system_attributes.source = Some(attribute.value.clone());
                    true
                }

                ("Text", "content") => {
                    system_attributes.content = Some(attribute.value.clone());
                    true
                }

                ("Input", "placeholder") => {
                    system_attributes.placeholder = Some(attribute.value.clone());
                    true
                }

                ("Input", "max_length") => {
                    match attribute.value.parse::<u32>() {
                        Ok(value) => system_attributes.max_length = Some(value),
                        Err(_) => self.invalid_value_warning(&attribute.name, &attribute.value, attribute.line, attribute.column),
                    }

                    true
                }

                ("Input", "min_length") => {
                    match attribute.value.parse::<u32>() {
                        Ok(value) => system_attributes.min_length = Some(value),
                        Err(_) => self.invalid_value_warning(&attribute.name, &attribute.value, attribute.line, attribute.column),
                    }

                    true
                }
                _ => false,
            };

            if !recognized {
                arbitrary_attributes.push((attribute.name, attribute.value));
            }
        }

        (system_attributes, arbitrary_attributes)
    }

    fn resolve_styles(&mut self, styles: Vec<Style>) -> (SystemStyles, Vec<(String, String)>) {
        let mut system_styles = SystemStyles::default();
        let mut arbitrary_styles = Vec::new();

        for style in styles {
            let recognized: bool = match style.name.as_str() {
                "layout" => {
                    match style.value.as_str() {
                        "simple" => system_styles.layout = Some(Layout::Simple),
                        "free" => system_styles.layout = Some(Layout::Free),
                        _ => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "direction" => {
                    match style.value.as_str() {
                        "horizontal" => system_styles.direction = Some(Direction::Horizontal),
                        "vertical" => system_styles.direction = Some(Direction::Vertical),
                        _ => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "alignment" => {
                    match style.value.as_str() {
                        "start" => system_styles.alignment = Some(Alignment::Start),
                        "center" => system_styles.alignment = Some(Alignment::Center),
                        "end" => system_styles.alignment = Some(Alignment::End),
                        _ => self.invalid_value_warning(&style.name, &style.value, style.line, style.column)
                    }

                    true
                }

                "alignment_x" => {
                    match style.value.as_str() {
                        "start" => system_styles.alignment_x = Some(Alignment::Start),
                        "center" => system_styles.alignment_x = Some(Alignment::Center),
                        "end" => system_styles.alignment_x = Some(Alignment::End),
                        _ => self.invalid_value_warning(&style.name, &style.value, style.line, style.column)
                    }

                    true
                }

                "alignment_y" => {
                    match style.value.as_str() {
                        "start" => system_styles.alignment_y = Some(Alignment::Start),
                        "center" => system_styles.alignment_y = Some(Alignment::Center),
                        "end" => system_styles.alignment_y = Some(Alignment::End),
                        _ => self.invalid_value_warning(&style.name, &style.value, style.line, style.column)
                    }

                    true
                }
                
                "width" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.width = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }
                    true
                }

                "height" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.height = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }
                    true
                }

                "content_color" => {
                    match Self::hex_to_rgb(&style.value) {
                        Some(color) => system_styles.content_color = Some(color),
                        None => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "background_color" => {
                    match Self::hex_to_rgb(&style.value) {
                        Some(color) => system_styles.background_color = Some(color),
                        None => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }
                _ => false,
            };

            if !recognized {
                arbitrary_styles.push((style.name, style.value));
            }
        }

        (system_styles, arbitrary_styles)
    }

    fn hex_to_rgb(value: &str) -> Option<RGB> {
        let hex = value.strip_prefix('#')?;

        if hex.len() != 6 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }

        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

        Some(RGB { r, g, b })
    }

    fn invalid_value_warning(&mut self, name: &str, value: &str, line: usize, column: usize) {
        self.warnings.push(SemanticWarning {
            message: format!(
                "'{}' has an invalid value '{}' and has been ignored",
                name, value
            ),
            line,
            column,
        });
    }
}