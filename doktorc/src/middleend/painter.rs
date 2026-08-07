use crate::middleend::scroller_ast::{ScrollerBlockNode, ScrollerDoktorNode};
use crate::middleend::painter_ast::{Rectangle, Text, Image, DrawStructure};

pub struct Painter;

impl Painter {
    pub fn new() -> Self {
        Painter
    }

    pub fn paint(&self, scroller_doktor_node: ScrollerDoktorNode) -> Vec<DrawStructure> {
        let mut draw_structures: Vec<DrawStructure> = Vec::new();

        for child in &scroller_doktor_node.children {
            self.paint_block(child, &mut draw_structures);
        }

        draw_structures
    }

    fn paint_block(&self, block: &ScrollerBlockNode, draw_structures: &mut Vec<DrawStructure>) {
        if let Some(draw_structure) = self.block_to_draw_structure(block) {
            draw_structures.push(draw_structure);
        }

        for child in &block.children {
            self.paint_block(child, draw_structures);
        }
    }

    fn block_to_draw_structure(&self, block: &ScrollerBlockNode) -> Option<DrawStructure> {
        match block.block_type.as_str() {
            "Image" => {
                let source: String = match &block.system_attributes.source {
                    Some(source) => source.clone(),
                    None => return None, // No source, so nothing to draw.
                };

                Some(DrawStructure::Image(Image {
                    id: block.id,
                    location: block.location,
                    width: block.size.width,
                    height: block.size.height,
                    clip: block.clip,
                    source,
                    background_color: block.system_styles.background_color,
                    border_top_color: block.system_styles.get_unambiguous_border_color("top"),
                    border_bottom_color: block.system_styles.get_unambiguous_border_color("bottom"),
                    border_left_color: block.system_styles.get_unambiguous_border_color("left"),
                    border_right_color: block.system_styles.get_unambiguous_border_color("right"),
                    border_top_size: block.system_styles.get_unambiguous_border_size("top"),
                    border_bottom_size: block.system_styles.get_unambiguous_border_size("bottom"),
                    border_left_size: block.system_styles.get_unambiguous_border_size("left"),
                    border_right_size: block.system_styles.get_unambiguous_border_size("right"),
                    border_top_type: block.system_styles.get_unambiguous_border_type("top"),
                    border_bottom_type: block.system_styles.get_unambiguous_border_type("bottom"),
                    border_left_type: block.system_styles.get_unambiguous_border_type("left"),
                    border_right_type: block.system_styles.get_unambiguous_border_type("right"),
                    opacity: block.system_styles.opacity,
                }))
            },

            "Text" => {
                let content: String = block.system_attributes.content.clone().unwrap_or_default();

                Some(DrawStructure::Text(Text {
                    id: block.id,
                    location: block.location,
                    clip: block.clip,
                    content,
                    content_color: block.system_styles.content_color,
                    content_size: block.system_styles.content_size,
                    content_font: block.system_styles.content_font,
                    background_color: block.system_styles.background_color,
                    border_top_color: block.system_styles.get_unambiguous_border_color("top"),
                    border_bottom_color: block.system_styles.get_unambiguous_border_color("bottom"),
                    border_left_color: block.system_styles.get_unambiguous_border_color("left"),
                    border_right_color: block.system_styles.get_unambiguous_border_color("right"),
                    border_top_size: block.system_styles.get_unambiguous_border_size("top"),
                    border_bottom_size: block.system_styles.get_unambiguous_border_size("bottom"),
                    border_left_size: block.system_styles.get_unambiguous_border_size("left"),
                    border_right_size: block.system_styles.get_unambiguous_border_size("right"),
                    border_top_type: block.system_styles.get_unambiguous_border_type("top"),
                    border_bottom_type: block.system_styles.get_unambiguous_border_type("bottom"),
                    border_left_type: block.system_styles.get_unambiguous_border_type("left"),
                    border_right_type: block.system_styles.get_unambiguous_border_type("right"),
                    opacity: block.system_styles.opacity,
                }))
            },

            _ => {
                Some(DrawStructure::Rectangle(Rectangle {
                    id: block.id,
                    location: block.location,
                    width: block.size.width,
                    height: block.size.height,
                    is_scrollable_x: block.is_scrollable_x,
                    is_scrollable_y: block.is_scrollable_y,
                    scrollable_width: block.scrollable_size.width,
                    scrollable_height: block.scrollable_size.height,
                    scroll_offset: block.scroll_offset,
                    clip: block.clip,
                    background_color: block.system_styles.background_color,
                    border_top_color: block.system_styles.get_unambiguous_border_color("top"),
                    border_bottom_color: block.system_styles.get_unambiguous_border_color("bottom"),
                    border_left_color: block.system_styles.get_unambiguous_border_color("left"),
                    border_right_color: block.system_styles.get_unambiguous_border_color("right"),
                    border_top_size: block.system_styles.get_unambiguous_border_size("top"),
                    border_bottom_size: block.system_styles.get_unambiguous_border_size("bottom"),
                    border_left_size: block.system_styles.get_unambiguous_border_size("left"),
                    border_right_size: block.system_styles.get_unambiguous_border_size("right"),
                    border_top_type: block.system_styles.get_unambiguous_border_type("top"),
                    border_bottom_type: block.system_styles.get_unambiguous_border_type("bottom"),
                    border_left_type: block.system_styles.get_unambiguous_border_type("left"),
                    border_right_type: block.system_styles.get_unambiguous_border_type("right"),
                    opacity: block.system_styles.opacity,
                }))
            },
        }
    }
}