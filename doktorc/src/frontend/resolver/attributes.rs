use crate::frontend::parser_ast::Attribute;

use crate::frontend::resolver::ast::system_attributes::SystemAttributes;
use crate::frontend::resolver::ast::invalids::ResolverWarning;

use crate::frontend::resolver::invalid_value_warning::invalid_value_warning;

pub struct Attributes {
    pub warnings: Vec<ResolverWarning>,
}

impl Attributes {
    pub fn new() -> Self {
        Attributes {
            warnings: Vec::new(),
        }
    }

    pub fn resolve_attributes(&mut self, block_type: &str, attributes: Vec<Attribute>) -> (SystemAttributes, Vec<(String, String)>) {
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
}