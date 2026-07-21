use crate::middleend::drawable::{DrawableBlockNode, DrawableDoktorNode};
use crate::middleend::painter_ast::{Rectangle, Text, Image, DrawStructure};

pub struct Painter;

impl Painter {
    pub fn new() -> Self {
        Painter
    }

    pub fn paint(&self, drawable_doktor_node: DrawableDoktorNode) -> Vec<DrawStructure> {
        let mut draw_structures: Vec<DrawStructure> = Vec::new();

        for child in &drawable_doktor_node.children {
            self.paint_block(child, &mut draw_structures);
        }

        draw_structures
    }

    fn paint_block(&self, block: &DrawableBlockNode, draw_structures: &mut Vec<DrawStructure>) {
        if let Some(draw_structure) = self.block_to_draw_structure(block) {
            draw_structures.push(draw_structure);
        }

        for child in &block.children {
            self.paint_block(child, draw_structures);
        }
    }

    fn block_to_draw_structure(&self, block: &DrawableBlockNode) -> Option<DrawStructure> {
        match block.block_type.as_str() {
            "Image" => {
                let source: String = match &block.system_attributes.source {
                    Some(source) => source.clone(),
                    None => return None, // No source, so nothing to draw.
                };

                Some(DrawStructure::Image(Image {
                    location: block.location,
                    width: block.size.width,
                    height: block.size.height,
                    source,
                }))
            },

            "Text" => {
                let content: String = block.system_attributes.content.clone().unwrap_or_default();

                Some(DrawStructure::Text(Text {
                    location: block.location,
                    content,
                    font_size: crate::middleend::layout::DEFAULT_FONT_SIZE,
                    color: block.system_styles.content_color.unwrap_or(crate::middleend::layout::DEFAULT_CONTENT_COLOR),
                }))
            },

            _ => {
                let color = block.system_styles.background_color.unwrap_or(crate::middleend::layout::DEFAULT_CONTENT_COLOR);

                Some(DrawStructure::Rectangle(Rectangle {
                    location: block.location,
                    width: block.size.width,
                    height: block.size.height,
                    color,
                }))
            },
        }
    }
}