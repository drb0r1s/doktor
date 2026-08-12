use std::collections::HashMap;

use crate::frontend::parser_ast::{Style, ParserBlockNode, ParserDoktorNode};

use crate::frontend::resolver::ast::collection::CollectionMap;
use crate::frontend::resolver::ast::nodes::{ResolverBlockNode, ResolverDoktorNode};
use crate::frontend::resolver::ast::invalids::{ResolverWarning, ResolverError};

use crate::frontend::resolver::collections::Collections;
use crate::frontend::resolver::attributes::Attributes;
use crate::frontend::resolver::styles::Styles;

const SYSTEM_BLOCK_TYPES: &[&str] = &["Group", "Image", "Text", "Input", "Collection", "Styles", "Style"];
const CHILDREN_BLOCK_TYPES: &[&str] = &["Group", "Collection", "Styles"];

pub struct Resolver {
    collections: Collections,
    attributes: Attributes,
    styles: Styles,
    warnings: Vec<ResolverWarning>,
    errors: Vec<ResolverError>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            collections: Collections::new(),
            attributes: Attributes::new(),
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

        let (system_attributes, arbitrary_attributes) = self.attributes.resolve_attributes(resolved_block_type, parser_block_node.attributes);
        self.warnings.extend(std::mem::take(&mut self.attributes.warnings));

        // Combining Style's block style properties with parser block's styles.
        let mut merged_styles: Vec<Style> = tag_styles.get(&parser_block_node.tag).cloned().unwrap_or_default();
        merged_styles.extend(parser_block_node.styles);
        
        let (system_styles, arbitrary_styles) = self.styles.resolve(merged_styles, &parser_block_node.block_type);
        self.warnings.extend(std::mem::take(&mut self.styles.warnings));

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
}