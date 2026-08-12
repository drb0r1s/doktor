use std::collections::HashMap;

use crate::frontend::parser_ast::{Style, ParserBlockNode};
use crate::frontend::resolver::ast::system_styles::{Layout, Direction, Alignment, BorderType, Overflow, SystemStyles};
use crate::frontend::resolver::ast::invalids::ResolverWarning;

use crate::frontend::resolver::invalid_value_warning::invalid_value_warning;

use crate::collections::rgb::RGB;
use crate::collections::font::Font;

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

    pub fn resolve(&mut self, styles: Vec<Style>, block_type: &String) -> (SystemStyles, Vec<(String, String)>) {
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