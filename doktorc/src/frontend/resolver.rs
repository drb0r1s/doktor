use std::fmt;
use std::collections::HashMap;

use colored::Colorize;

use crate::frontend::parser_ast::{Attribute, Style, ParserBlockNode, ParserDoktorNode};
use crate::frontend::resolver_ast::{RGB, Layout, Direction, Alignment, parse_font, BorderType, Overflow, ParamType, parse_param_type, Collection, CollectionMap, SystemAttributes, SystemStyles, ResolverBlockNode, ResolverDoktorNode};

use crate::data::prefix::get_prefix;

#[derive(Debug, Clone, PartialEq)]
pub struct ResolverWarning {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for ResolverWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} [{}:{}]: {}.",
            get_prefix(), "(Resolver)".magenta().italic(), "Warning".on_yellow(), self.line, self.column, self.message
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolverError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for ResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} [{}:{}]: {}.",
            get_prefix(), "(Resolver)".magenta().italic(), "Error".on_red(), self.line, self.column, self.message
        )
    }
}

impl std::error::Error for ResolverError {}

const SYSTEM_BLOCK_TYPES: &[&str] = &["Group", "Image", "Text", "Input", "Collection", "Styles", "Style"];
const CHILDREN_BLOCK_TYPES: &[&str] = &["Group", "Collection", "Styles"];

pub struct Resolver {
    warnings: Vec<ResolverWarning>,
    errors: Vec<ResolverError>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn resolve(mut self, parser_doktor_node: ParserDoktorNode) -> (ResolverDoktorNode, Vec<ResolverWarning>, Vec<ResolverError>) {
        let tag_styles: HashMap<String, Vec<Style>> = Self::collect_tag_styles(&parser_doktor_node.children);
        
        let collections: CollectionMap = self.collect_collections(&parser_doktor_node.children);
        let top_level_filtered: Vec<ParserBlockNode> = parser_doktor_node.children.into_iter().filter(|child| child.block_type != "Styles" && child.block_type != "Collection").collect();
        let mut expansion_stack: Vec<String> = Vec::new();

        let expanded = self.expand_collections(top_level_filtered, &collections, &mut expansion_stack);
        
        let children = self.filter_style_blocks(expanded, &tag_styles);

        (ResolverDoktorNode { children }, self.warnings, self.errors)
    }

    fn collect_collections(&mut self, children: &[ParserBlockNode]) -> CollectionMap {
        let mut collections: CollectionMap = HashMap::new();

        for block in children {
            if block.block_type == "Collection" {
                if block.tag.is_empty() {
                    self.errors.push(ResolverError {
                        message: format!("\"Collection\" block must have a tag, it is ignored otherwise"),
                        line: block.line,
                        column: block.column,
                    });

                    continue;
                }

                if block.children.len() > 1 {
                    self.errors.push(ResolverError {
                        message: format!("Collection \"{}\" must return exactly one top-level block, other blocks are ignored", block.tag),
                        line: block.line,
                        column: block.column,
                    });
                }

                if !block.tag.chars().next().is_some_and(|character| character.is_uppercase()) {
                    self.warnings.push(ResolverWarning {
                        message: format!("It is recommended to start a \"Collection\" block \"{}\" with a capital letter", block.tag),
                        line: block.line,
                        column: block.column,
                    });
                }

                if let Some(first_child) = block.children.first() {
                    let (attributes, styles) = self.parse_collection_params(block);

                    let collection: Collection = Collection {
                        body: first_child.clone(),
                        attributes,
                        styles,
                    };

                    collections.insert(block.tag.clone(), collection);
                }
                
                else {
                    self.warnings.push(ResolverWarning {
                        message: format!("Collection \"{}\" has no body, it is ignored", block.tag),
                        line: block.line,
                        column: block.column,
                    });
                }
            }
        }

        collections
    }

    fn parse_collection_params(&mut self, block: &ParserBlockNode) -> (HashMap<String, ParamType>, HashMap<String, ParamType>) {
        let mut attributes: HashMap<String, ParamType> = HashMap::new();
        let mut styles: HashMap<String, ParamType> = HashMap::new();

        for attribute in &block.attributes {
            match parse_param_type(&attribute.value) {
                Some(param_type) => { attributes.insert(attribute.name.clone(), param_type); }
                None => self.invalid_value_warning(&attribute.name, &attribute.value, attribute.line, attribute.column),
            }
        }

        for style in &block.styles {
            match parse_param_type(&style.value) {
                Some(param_type) => { styles.insert(style.name.clone(), param_type); }
                None => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
            }
        }

        (attributes, styles)
    }

    fn expand_collections(&mut self, children: Vec<ParserBlockNode>, collections: &CollectionMap, expansion_stack: &mut Vec<String>) -> Vec<ParserBlockNode> {
        children.into_iter().map(|block| self.expand_block(block, collections, expansion_stack)).collect()
    }

    fn expand_block(&mut self, mut block: ParserBlockNode, collections: &CollectionMap, expansion_stack: &mut Vec<String>) -> ParserBlockNode {
        if let Some(collection) = collections.get(&block.block_type) {
            if expansion_stack.contains(&block.block_type) {
                self.errors.push(ResolverError {
                    message: format!("Cycle detected calling collection \"{}\", it is ignored.", block.block_type),
                    line: block.line,
                    column: block.column,
                });

                return block;
            }

            let values = self.validate_value_types(&block, collection);

            let mut substituted_body = collection.body.clone();
            Self::substitute_block(&mut substituted_body, &values);

            expansion_stack.push(block.block_type.clone());
            let expanded = self.expand_block(substituted_body, collections, expansion_stack);
            expansion_stack.pop();

            return expanded;
        }

        block.children = self.expand_collections(block.children, collections, expansion_stack);

        block
    }

    fn substitute_variables(text: &str, values: &HashMap<String, String>) -> String {
        let mut result = String::new();
        let mut characters = text.chars().peekable();

        while let Some(character) = characters.next() {
            if character == '*' {
                let mut name = String::new();

                while let Some(&next) = characters.peek() {
                    if next.is_ascii_alphabetic() || next == '_' {
                        name.push(next);
                        characters.next();
                    }
                    
                    else {
                        break;
                    }
                }

                if name.is_empty() {
                    result.push('*'); // Stray '*' with no valid identifier after it, kept literally.
                }
                
                else {
                    result.push_str(values.get(&name).map(String::as_str).unwrap_or("NULL"));
                }
            }
            
            else {
                result.push(character);
            }
        }

        result
    }

    fn validate_value_types(&mut self, block: &ParserBlockNode, collection: &Collection) -> HashMap<String, String> {
        let mut values: HashMap<String, String> = HashMap::new();

        for attribute in &block.attributes {
            match collection.attributes.get(&attribute.name) {
                Some(param_type) => {
                    if Self::value_matches_type(&attribute.value, *param_type) {
                        values.insert(attribute.name.clone(), attribute.value.clone());
                    }
                    
                    else {
                        self.invalid_value_warning(&attribute.name, &attribute.value, attribute.line, attribute.column);
                    }
                }

                None => self.invalid_value_warning(&attribute.name, &attribute.value, attribute.line, attribute.column),
            }
        }

        for style in &block.styles {
            match collection.styles.get(&style.name) {
                Some(param_type) => {
                    if Self::value_matches_type(&style.value, *param_type) {
                        values.insert(style.name.clone(), style.value.clone());
                    } else {
                        self.invalid_value_warning(&style.name, &style.value, style.line, style.column);
                    }
                }

                None => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
            }
        }

        values
    }

    fn value_matches_type(value: &str, param_type: ParamType) -> bool {
        match param_type {
            ParamType::Text => true,
            ParamType::Number => value.parse::<f32>().is_ok(),
            ParamType::Bool => matches!(value, "true" | "false" | "1" | "0"),
            ParamType::Color => Self::hex_to_rgb(value).is_some(),
        }
    }

    fn substitute_block(block: &mut ParserBlockNode, values: &HashMap<String, String>) {
        for attribute in &mut block.attributes {
            attribute.value = Self::substitute_variables(&attribute.value, values);
        }

        for style in &mut block.styles {
            style.value = Self::substitute_variables(&style.value, values);
        }

        for child in &mut block.children {
            Self::substitute_block(child, values);
        }
    }

    fn collect_tag_styles(children: &[ParserBlockNode]) -> HashMap<String, Vec<Style>> {
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

    fn filter_style_blocks(&mut self, children: Vec<ParserBlockNode>, tag_styles: &HashMap<String, Vec<Style>>) -> Vec<ResolverBlockNode> {
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

                _ => Some(self.resolve_block(child, tag_styles))
            }
        }).collect()
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
            self.filter_style_blocks(parser_block_node.children, tag_styles)
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
                        _ => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "direction" => {
                    match style.value.as_str() {
                        "horizontal" => system_styles.direction = Direction::Horizontal,
                        "vertical" => system_styles.direction = Direction::Vertical,
                        _ => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "alignment" => {
                    match style.value.as_str() {
                        "start" => system_styles.alignment = Alignment::Start,
                        "center" => system_styles.alignment = Alignment::Center,
                        "end" => system_styles.alignment = Alignment::End,
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
                    if let Some(stripped) = style.value.strip_suffix('%') {
                        match stripped.parse::<f32>() {
                            Ok(value) => {
                                system_styles.width = 0.0; // Placeholder.
                                system_styles.width_percent = Some(value / 100.0)
                            },
                            
                            Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                        }
                    } else {
                        match style.value.parse::<f32>() {
                            Ok(value) => system_styles.width = value,
                            Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
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

                            Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                        }
                    } else {
                        match style.value.parse::<f32>() {
                            Ok(value) => system_styles.height = value,
                            Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                        }
                    }

                    true
                }

                "lock_dimensions" => {
                    match style.value.as_str() {
                        "true" | "1" => system_styles.lock_dimensions = true,
                        "false" | "0" => system_styles.lock_dimensions = false,
                        _ => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "lock_width" => {
                    match style.value.as_str() {
                        "true" | "1" => system_styles.lock_width = Some(true),
                        "false" | "0" => system_styles.lock_width = Some(false),
                        _ => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "lock_height" => {
                    match style.value.as_str() {
                        "true" | "1" => system_styles.lock_height = Some(true),
                        "false" | "0" => system_styles.lock_height = Some(false),
                        _ => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "position" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.position = value,
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "position_x" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.position_x = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "position_y" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.position_y = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "content_color" => {
                    match Self::hex_to_rgb(&style.value) {
                        Some(color) => system_styles.content_color = color,
                        None => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "content_size" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.content_size = value,
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "content_font" => {
                    match parse_font(&style.value) {
                        Some(font) => system_styles.content_font = font,
                        None => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "background_color" => {
                    match Self::hex_to_rgb(&style.value) {
                        Some(color) => system_styles.background_color = color,
                        None => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "border_color" => {
                    match Self::hex_to_rgb(&style.value) {
                        Some(color) => system_styles.border_color = color,
                        None => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "border_top_color" => {
                    match Self::hex_to_rgb(&style.value) {
                        Some(color) => system_styles.border_top_color = Some(color),
                        None => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "border_bottom_color" => {
                    match Self::hex_to_rgb(&style.value) {
                        Some(color) => system_styles.border_bottom_color = Some(color),
                        None => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "border_left_color" => {
                    match Self::hex_to_rgb(&style.value) {
                        Some(color) => system_styles.border_left_color = Some(color),
                        None => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "border_right_color" => {
                    match Self::hex_to_rgb(&style.value) {
                        Some(color) => system_styles.border_right_color = Some(color),
                        None => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "border_size" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.border_size = value,
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "border_top_size" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.border_top_size = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "border_bottom_size" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.border_bottom_size = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "border_left_size" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.border_left_size = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "border_right_size" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.border_right_size = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "border_type" => {
                    match style.value.as_str() {
                        "none" => system_styles.border_type = BorderType::None,
                        "solid" => system_styles.border_type = BorderType::Solid,
                        "dashed" => system_styles.border_type = BorderType::Dashed,
                        "dotted" => system_styles.border_type = BorderType::Dotted,
                        _ => self.invalid_value_warning(&style.name, &style.value, style.line, style.column)
                    }

                    true
                }

                "border_top_type" => {
                    match style.value.as_str() {
                        "none" => system_styles.border_top_type = Some(BorderType::None),
                        "solid" => system_styles.border_top_type = Some(BorderType::Solid),
                        "dashed" => system_styles.border_top_type = Some(BorderType::Dashed),
                        "dotted" => system_styles.border_top_type = Some(BorderType::Dotted),
                        _ => self.invalid_value_warning(&style.name, &style.value, style.line, style.column)
                    }

                    true
                }

                "border_bottom_type" => {
                    match style.value.as_str() {
                        "none" => system_styles.border_bottom_type = Some(BorderType::None),
                        "solid" => system_styles.border_bottom_type = Some(BorderType::Solid),
                        "dashed" => system_styles.border_bottom_type = Some(BorderType::Dashed),
                        "dotted" => system_styles.border_bottom_type = Some(BorderType::Dotted),
                        _ => self.invalid_value_warning(&style.name, &style.value, style.line, style.column)
                    }

                    true
                }

                "border_left_type" => {
                    match style.value.as_str() {
                        "none" => system_styles.border_left_type = Some(BorderType::None),
                        "solid" => system_styles.border_left_type = Some(BorderType::Solid),
                        "dashed" => system_styles.border_left_type = Some(BorderType::Dashed),
                        "dotted" => system_styles.border_left_type = Some(BorderType::Dotted),
                        _ => self.invalid_value_warning(&style.name, &style.value, style.line, style.column)
                    }

                    true
                }

                "border_right_type" => {
                    match style.value.as_str() {
                        "none" => system_styles.border_right_type = Some(BorderType::None),
                        "solid" => system_styles.border_right_type = Some(BorderType::Solid),
                        "dashed" => system_styles.border_right_type = Some(BorderType::Dashed),
                        "dotted" => system_styles.border_right_type = Some(BorderType::Dotted),
                        _ => self.invalid_value_warning(&style.name, &style.value, style.line, style.column)
                    }

                    true
                }

                "opacity" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.opacity = value,
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "spacing" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.spacing = value,
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "spacing_top" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.spacing_top = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "spacing_bottom" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.spacing_bottom = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "spacing_left" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.spacing_left = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "spacing_right" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.spacing_right = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "margin" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.margin = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "margin_top" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.margin_top = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "margin_bottom" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.margin_bottom = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "margin_left" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.margin_left = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "margin_right" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.margin_right = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "padding" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.padding = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "padding_top" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.padding_top = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "padding_bottom" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.padding_bottom = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "padding_left" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.padding_left = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "padding_right" => {
                    match style.value.parse::<f32>() {
                        Ok(value) => system_styles.padding_right = Some(value),
                        Err(_) => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "overflow" => {
                    match style.value.as_str() {
                        "true" | "1" => system_styles.overflow = Overflow::True,
                        "false" | "0" => system_styles.overflow = Overflow::False,
                        "scroll" => system_styles.overflow = Overflow::Scroll,
                        _ => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "overflow_x" => {
                    match style.value.as_str() {
                        "true" | "1" => system_styles.overflow_x = Some(Overflow::True),
                        "false" | "0" => system_styles.overflow_x = Some(Overflow::False),
                        "scroll" => system_styles.overflow_x = Some(Overflow::Scroll),
                        _ => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
                    }

                    true
                }

                "overflow_y" => {
                    match style.value.as_str() {
                        "true" | "1" => system_styles.overflow_y = Some(Overflow::True),
                        "false" | "0" => system_styles.overflow_y = Some(Overflow::False),
                        "scroll" => system_styles.overflow_y = Some(Overflow::Scroll),
                        _ => self.invalid_value_warning(&style.name, &style.value, style.line, style.column),
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

    fn hex_to_rgb(value: &str) -> Option<RGB> {
        let hex = value.strip_prefix('#')?;

        if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }

        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

                Some(RGB { r, g, b, a: 255 })
            }

            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;

                Some(RGB { r, g, b, a })
            }

            _ => None
        }
    }

    fn invalid_value_warning(&mut self, name: &str, value: &str, line: usize, column: usize) {
        self.warnings.push(ResolverWarning {
            message: format!(
                "\"{}\" has an invalid value \"{}\" and has been ignored",
                name, value
            ),
            line,
            column,
        });
    }
}