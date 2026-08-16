use std::collections::HashMap;
use uuid::Uuid;

use crate::frontend::parser_ast::ParserBlockNode;

use crate::frontend::resolver::ast::collection::{ParamType, parse_param_type, Collection, CollectionMap};
use crate::frontend::resolver::ast::invalids::{ResolverWarning, ResolverError};
use crate::frontend::resolver::invalid_value_warning::invalid_value_warning;

use crate::collections::rgb::RGB;

pub struct Collections {
    pub warnings: Vec<ResolverWarning>,
    pub errors: Vec<ResolverError>,
}

impl Collections {
    pub fn new() -> Self {
        Collections {
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }
    
    pub fn collect(&mut self, children: &[ParserBlockNode]) -> CollectionMap {
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
                None => self.warnings.push(invalid_value_warning(&attribute.name, &attribute.value, attribute.line, attribute.column)),
            }
        }

        for style in &block.styles {
            match parse_param_type(&style.value) {
                Some(param_type) => { styles.insert(style.name.clone(), param_type); }
                None => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
            }
        }

        (attributes, styles)
    }

    pub fn expand(&mut self, children: Vec<ParserBlockNode>, collections: &CollectionMap, expansion_stack: &mut Vec<String>) -> Vec<ParserBlockNode> {
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
            Self::regenerate_ids(&mut substituted_body); // We need to make sure Collections produce new IDs, to prevent collisions.
            Self::substitute_block(&mut substituted_body, &values);

            expansion_stack.push(block.block_type.clone());
            let expanded = self.expand_block(substituted_body, collections, expansion_stack);
            expansion_stack.pop();

            return expanded;
        }

        block.children = self.expand(block.children, collections, expansion_stack);

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
                        self.warnings.push(invalid_value_warning(&attribute.name, &attribute.value, attribute.line, attribute.column));
                    }
                }

                None => self.warnings.push(invalid_value_warning(&attribute.name, &attribute.value, attribute.line, attribute.column)),
            }
        }

        for style in &block.styles {
            match collection.styles.get(&style.name) {
                Some(param_type) => {
                    if Self::value_matches_type(&style.value, *param_type) {
                        values.insert(style.name.clone(), style.value.clone());
                    } else {
                        self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column));
                    }
                }

                None => self.warnings.push(invalid_value_warning(&style.name, &style.value, style.line, style.column)),
            }
        }

        values
    }

    fn value_matches_type(value: &str, param_type: ParamType) -> bool {
        match param_type {
            ParamType::Text => true,
            ParamType::Number => value.parse::<f32>().is_ok(),
            ParamType::Bool => matches!(value, "true" | "false" | "1" | "0"),
            ParamType::Color => RGB::hex_to_rgb(value).is_some(),
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

    fn regenerate_ids(block: &mut ParserBlockNode) {
        block.id = Uuid::new_v4();

        for child in &mut block.children {
            Self::regenerate_ids(child);
        }
    }
}