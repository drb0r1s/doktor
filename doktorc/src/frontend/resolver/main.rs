use std::collections::HashMap;

use crate::frontend::parser_ast::{Attribute, Style, ParserBlockNode, ParserDoktorNode};

use crate::frontend::resolver::ast::system_attributes::SystemAttributes;
use crate::frontend::resolver::ast::system_styles::{Layout, Direction, Alignment, BorderType, Overflow, SystemStyles};
use crate::frontend::resolver::ast::collection::CollectionMap;
use crate::frontend::resolver::ast::nodes::{ResolverBlockNode, ResolverDoktorNode};
use crate::frontend::resolver::ast::invalids::{ResolverWarning, ResolverError};

use crate::frontend::resolver::collections::Collections;
use crate::frontend::resolver::styles::Styles;
use crate::frontend::resolver::invalid_value_warning::invalid_value_warning;

use crate::collections::rgb::RGB;
use crate::collections::font::Font;

const SYSTEM_BLOCK_TYPES: &[&str] = &["Group", "Image", "Text", "Input", "Collection", "Styles", "Style"];
const CHILDREN_BLOCK_TYPES: &[&str] = &["Group", "Collection", "Styles"];

pub struct Resolver {
    collections: Collections,
    styles: Styles,
    warnings: Vec<ResolverWarning>,
    errors: Vec<ResolverError>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            collections: Collections::new(),
            styles: Styles::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn resolve(mut self, parser_doktor_node: ParserDoktorNode) -> (ResolverDoktorNode, Vec<ResolverWarning>, Vec<ResolverError>) {
        let tag_styles: HashMap<String, Vec<Style>> = Styles::collect_tag_styles(&parser_doktor_node.children);
        
        let collections: CollectionMap = self.collections.collect(&parser_doktor_node.children);

        let top_level_filtered: Vec<ParserBlockNode> = parser_doktor_node.children.into_iter().filter(|child| child.block_type != "Styles" && child.block_type != "Collection").collect();
        let mut expansion_stack: Vec<String> = Vec::new();

        let expanded = self.collections.expand(top_level_filtered, &collections, &mut expansion_stack);
        
        let filtered_children = self.styles.filter_style_blocks(expanded);
        let children = filtered_children.into_iter().map(|child| self.resolve_block(child, &tag_styles)).collect();

        // We need to collect all the warnings and errors from sub-structs.
        self.warnings.extend(std::mem::take(&mut self.collections.warnings));
        self.warnings.extend(std::mem::take(&mut self.styles.warnings));

        self.errors.extend(std::mem::take(&mut self.collections.errors));

        (ResolverDoktorNode { children }, self.warnings, self.errors)
    }

    fn resolve_block(&mut self, parser_block_node: ParserBlockNode, tag_styles: &HashMap<String, Vec<Style>>) -> ResolverBlockNode {        
        let resolved_block_type: &str = if SYSTEM_BLOCK_TYPES.contains(&parser_block_node.block_type.as_str()) {
            &parser_block_node.block_type
        } else {
            self.errors.push(ResolverError {
                message: format!(
                    "Unrecognized block type \"{}\", it will be treated as a \"Group\"",
                    parser_block_node.block_type
                ),
                line: parser_block_node.line,
                column: parser_block_node.column,
            });

            "Group"
        };

        let (system_attributes, arbitrary_attributes) = self.resolve_attributes(resolved_block_type, parser_block_node.attributes);
        
        // Combining Style's block style properties with parser block's styles.
        let mut merged_styles: Vec<Style> = tag_styles.get(&parser_block_node.tag).cloned().unwrap_or_default();
        merged_styles.extend(parser_block_node.styles);
        
        let (system_styles, arbitrary_styles) = self.resolve_styles(merged_styles, &parser_block_node.block_type);

        let children = if !parser_block_node.children.is_empty() && !CHILDREN_BLOCK_TYPES.contains(&resolved_block_type) {
            self.errors.push(ResolverError {
                message: format!(
                    "Blocks of type \"{}\" cannot have children, they will be ignored",
                    resolved_block_type
                ),
                line: parser_block_node.line,
                column: parser_block_node.column,
            });

            Vec::new()
        } else {
            let filtered_children = self.styles.filter_style_blocks(parser_block_node.children);
            self.warnings.extend(std::mem::take(&mut self.styles.warnings));

            filtered_children.into_iter().map(|child| self.resolve_block(child, &tag_styles)).collect()
        };

        ResolverBlockNode {
            id: parser_block_node.id,
            block_type: parser_block_node.block_type,
            tag: parser_block_node.tag,
            system_attributes,
            arbitrary_attributes,
            system_styles,
            arbitrary_styles,
            children,
            line: parser_block_node.line,
            column: parser_block_node.column,
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
                        Err(_) => self.warnings.push(invalid_value_warning(&attribute.name, &attribute.value, attribute.line, attribute.column)),
                    }

                    true
                }

                ("Input", "min_length") => {
                    match attribute.value.parse::<u32>() {
                        Ok(value) => system_attributes.min_length = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&attribute.name, &attribute.value, attribute.line, attribute.column)),
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

    fn resolve_styles(&mut self, styles: Vec<Style>, block_type: &String) -> (SystemStyles, Vec<(String, String)>) {
        let is_text = block_type == "Text";
        
        let mut system_styles = SystemStyles::default(is_text);
        let mut arbitrary_styles = Vec::new();

        for style in styles {
            let recognized: bool = match style.name.as_str() {
                "layout" => {
                    match style.value.as_str() {
                        "simple" => system_styles.layout = Layout::Simple,
                        "free" => system_styles.layout = Layout::Free,
                        _ => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "direction" => {
                    match style.value.as_str() {
                        "horizontal" => system_styles.direction = Direction::Horizontal,
                        "vertical" => system_styles.direction = Direction::Vertical,
                        _ => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "alignment" => {
                    match style.value.as_str() {
                        "start" => system_styles.alignment = Alignment::Start,
                        "center" => system_styles.alignment = Alignment::Center,
                        "end" => system_styles.alignment = Alignment::End,
                        _ => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column))
                    }

                    true
                }

                "alignment_x" => {
                    match style.value.as_str() {
                        "start" => system_styles.alignment_x = Some(Alignment::Start),
                        "center" => system_styles.alignment_x = Some(Alignment::Center),
                        "end" => system_styles.alignment_x = Some(Alignment::End),
                        _ => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column))
                    }

                    true
                }

                "alignment_y" => {
                    match style.value.as_str() {
                        "start" => system_styles.alignment_y = Some(Alignment::Start),
                        "center" => system_styles.alignment_y = Some(Alignment::Center),
                        "end" => system_styles.alignment_y = Some(Alignment::End),
                        _ => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column))
                    }

                    true
                }
                
                "width" => {
                    if let Some(stripped) = style.value.strip_suffix('%') {
                        match stripped.parse::<f32>() {
                            Ok(value) => {
                                system_styles.width = 0.0; // Placeholder.
                                system_styles.width_percent = Some(value / 100.0)
                            },
                            
                            Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                        }
                    } else {
                        match style.value.parse::<f32>() {
                            Ok(value) => system_styles.width = value,
                            Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                        }
                    }

                    true
                }

                "height" => {
                    if let Some(stripped) = style.value.strip_suffix('%') {
                        match stripped.parse::<f32>() {
                            Ok(value) => {
                                system_styles.height = 0.0; // Placeholder.
                                system_styles.height_percent = Some(value / 100.0)
                            },

                            Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                        }
                    } else {
                        match style.value.parse::<f32>() {
                            Ok(value) => system_styles.height = value,
                            Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                        }
                    }

                    true
                }

                "lock_dimensions" => {
                    match style.value.as_str() {
                        "true" | "1" => system_styles.lock_dimensions = true,
                        "false" | "0" => system_styles.lock_dimensions = false,
                        _ => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "lock_width" => {
                    match style.value.as_str() {
                        "true" | "1" => system_styles.lock_width = Some(true),
                        "false" | "0" => system_styles.lock_width = Some(false),
                        _ => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "lock_height" => {
                    match style.value.as_str() {
                        "true" | "1" => system_styles.lock_height = Some(true),
                        "false" | "0" => system_styles.lock_height = Some(false),
                        _ => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "position" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.position = value,
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "position_x" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.position_x = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "position_y" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.position_y = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "content_color" => {
                    match RGB::hex_to_rgb(&style.value) {
                        Some(color) => system_styles.content_color = color,
                        None => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "content_size" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.content_size = value,
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "content_font" => {
                    match Font::parse_font(&style.value) {
                        Some(font) => system_styles.content_font = font,
                        None => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "background_color" => {
                    match RGB::hex_to_rgb(&style.value) {
                        Some(color) => system_styles.background_color = color,
                        None => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "border_color" => {
                    match RGB::hex_to_rgb(&style.value) {
                        Some(color) => system_styles.border_color = color,
                        None => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "border_top_color" => {
                    match RGB::hex_to_rgb(&style.value) {
                        Some(color) => system_styles.border_top_color = Some(color),
                        None => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "border_bottom_color" => {
                    match RGB::hex_to_rgb(&style.value) {
                        Some(color) => system_styles.border_bottom_color = Some(color),
                        None => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "border_left_color" => {
                    match RGB::hex_to_rgb(&style.value) {
                        Some(color) => system_styles.border_left_color = Some(color),
                        None => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "border_right_color" => {
                    match RGB::hex_to_rgb(&style.value) {
                        Some(color) => system_styles.border_right_color = Some(color),
                        None => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "border_size" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.border_size = value,
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "border_top_size" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.border_top_size = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "border_bottom_size" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.border_bottom_size = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "border_left_size" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.border_left_size = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "border_right_size" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.border_right_size = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "border_type" => {
                    match style.value.as_str() {
                        "none" => system_styles.border_type = BorderType::None,
                        "solid" => system_styles.border_type = BorderType::Solid,
                        "dashed" => system_styles.border_type = BorderType::Dashed,
                        "dotted" => system_styles.border_type = BorderType::Dotted,
                        _ => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column))
                    }

                    true
                }

                "border_top_type" => {
                    match style.value.as_str() {
                        "none" => system_styles.border_top_type = Some(BorderType::None),
                        "solid" => system_styles.border_top_type = Some(BorderType::Solid),
                        "dashed" => system_styles.border_top_type = Some(BorderType::Dashed),
                        "dotted" => system_styles.border_top_type = Some(BorderType::Dotted),
                        _ => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column))
                    }

                    true
                }

                "border_bottom_type" => {
                    match style.value.as_str() {
                        "none" => system_styles.border_bottom_type = Some(BorderType::None),
                        "solid" => system_styles.border_bottom_type = Some(BorderType::Solid),
                        "dashed" => system_styles.border_bottom_type = Some(BorderType::Dashed),
                        "dotted" => system_styles.border_bottom_type = Some(BorderType::Dotted),
                        _ => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column))
                    }

                    true
                }

                "border_left_type" => {
                    match style.value.as_str() {
                        "none" => system_styles.border_left_type = Some(BorderType::None),
                        "solid" => system_styles.border_left_type = Some(BorderType::Solid),
                        "dashed" => system_styles.border_left_type = Some(BorderType::Dashed),
                        "dotted" => system_styles.border_left_type = Some(BorderType::Dotted),
                        _ => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column))
                    }

                    true
                }

                "border_right_type" => {
                    match style.value.as_str() {
                        "none" => system_styles.border_right_type = Some(BorderType::None),
                        "solid" => system_styles.border_right_type = Some(BorderType::Solid),
                        "dashed" => system_styles.border_right_type = Some(BorderType::Dashed),
                        "dotted" => system_styles.border_right_type = Some(BorderType::Dotted),
                        _ => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column))
                    }

                    true
                }

                "opacity" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.opacity = value,
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "spacing" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.spacing = value,
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "spacing_top" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.spacing_top = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "spacing_bottom" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.spacing_bottom = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "spacing_left" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.spacing_left = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "spacing_right" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.spacing_right = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "margin" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.margin = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "margin_top" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.margin_top = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "margin_bottom" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.margin_bottom = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "margin_left" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.margin_left = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "margin_right" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.margin_right = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "padding" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.padding = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "padding_top" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.padding_top = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "padding_bottom" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.padding_bottom = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "padding_left" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.padding_left = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "padding_right" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.padding_right = Some(value),
                        Err(_) => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "overflow" => {
                    match style.value.as_str() {
                        "true" | "1" => system_styles.overflow = Overflow::True,
                        "false" | "0" => system_styles.overflow = Overflow::False,
                        "scroll" => system_styles.overflow = Overflow::Scroll,
                        _ => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "overflow_x" => {
                    match style.value.as_str() {
                        "true" | "1" => system_styles.overflow_x = Some(Overflow::True),
                        "false" | "0" => system_styles.overflow_x = Some(Overflow::False),
                        "scroll" => system_styles.overflow_x = Some(Overflow::Scroll),
                        _ => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                "overflow_y" => {
                    match style.value.as_str() {
                        "true" | "1" => system_styles.overflow_y = Some(Overflow::True),
                        "false" | "0" => system_styles.overflow_y = Some(Overflow::False),
                        "scroll" => system_styles.overflow_y = Some(Overflow::Scroll),
                        _ => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
                    }

                    true
                }

                _ => false
            };

            if !recognized {
                arbitrary_styles.push((style.name, style.value));
            }
        }

        (system_styles, arbitrary_styles)
    }

    
}