use std::collections::HashMap;

use crate::frontend::resolver::ast::system_styles::Overflow;

use crate::middleend::shaper_ast::{Location, Size, Clip, ShaperBlockNode, ShaperDoktorNode};
use crate::middleend::scroller_ast::{ScrollerBlockNode, ScrollerDoktorNode};

pub struct Scroller;

impl Scroller {
    pub fn new() -> Self {
        Scroller
    }
    
    pub fn scroll(&self, shaper_doktor_node: &ShaperDoktorNode, viewport_clip: Clip, scroll_offsets: &HashMap<u32, Location>) -> ScrollerDoktorNode {
        let children: Vec<ScrollerBlockNode> = shaper_doktor_node.children.iter().map(|child| Self::block_scroll(child, Location { x: 0.0, y: 0.0 }, viewport_clip, scroll_offsets)).collect();

        ScrollerDoktorNode { children }
    }

    fn block_scroll(shaper_block_node: &ShaperBlockNode, inherited_offset: Location, inherited_clip: Clip, scroll_offsets: &HashMap<u32, Location>) -> ScrollerBlockNode {
        // Applying any offset that block has inherited from its ancestors.
        let location: Location = Location {
            x: shaper_block_node.location.x - inherited_offset.x,
            y: shaper_block_node.location.y - inherited_offset.y,
        };

        let overflow_x: Overflow = shaper_block_node.system_styles.get_unambiguous_overflow("x");
        let overflow_y: Overflow = shaper_block_node.system_styles.get_unambiguous_overflow("y");

        let clip: Clip = Clip {
            x: if overflow_x == Overflow::False || overflow_x == Overflow::Scroll {
                intersect_range(inherited_clip.x, (location.x, location.x + shaper_block_node.size.width))
            } else {
                inherited_clip.x
            },

            y: if overflow_y == Overflow::False || overflow_y == Overflow::Scroll {
                intersect_range(inherited_clip.y, (location.y, location.y + shaper_block_node.size.height))
            } else {
                inherited_clip.y
            },
        };

        // Determining if this block has scrolling.
        let is_overflow_x_scroll: bool = shaper_block_node.system_styles.get_unambiguous_overflow("x") == Overflow::Scroll;
        let is_overflow_y_scroll: bool = shaper_block_node.system_styles.get_unambiguous_overflow("y") == Overflow::Scroll;

        let content_size: Size = Self::get_content_size(&shaper_block_node.children);

        let overflow_x_exists: bool = content_size.width > shaper_block_node.size.width;
        let overflow_y_exists: bool = content_size.height > shaper_block_node.size.height;

        let is_scrollable_x: bool = is_overflow_x_scroll && overflow_x_exists;
        let is_scrollable_y: bool = is_overflow_y_scroll && overflow_y_exists;

        let block_offset: Location = scroll_offsets.get(&shaper_block_node.id).copied().unwrap_or(Location { x: 0.0, y: 0.0 });

        let max_offset_x: f32 = (content_size.width - shaper_block_node.size.width).max(0.0);
        let max_offset_y: f32 = (content_size.height - shaper_block_node.size.height).max(0.0);

        let block_offset: Location = Location {
            x: block_offset.x.clamp(0.0, max_offset_x),
            y: block_offset.y.clamp(0.0, max_offset_y),
        };

        let child_offset = Location {
            x: inherited_offset.x + if is_scrollable_x { block_offset.x } else { 0.0 },
            y: inherited_offset.y + if is_overflow_y_scroll { block_offset.y } else { 0.0 },
        };

        let children: Vec<ScrollerBlockNode> = shaper_block_node.children.iter().map(|child| Self::block_scroll(child, child_offset, clip.clone(), scroll_offsets)).collect();
        
        ScrollerBlockNode {
            id: shaper_block_node.id,
            block_type: shaper_block_node.block_type.clone(),
            tag: shaper_block_node.tag.clone(),
            system_attributes: shaper_block_node.system_attributes.clone(),
            arbitrary_attributes: shaper_block_node.arbitrary_attributes.clone(),
            system_styles: shaper_block_node.system_styles.clone(),
            arbitrary_styles: shaper_block_node.arbitrary_styles.clone(),
            size: shaper_block_node.size,
            location,
            clip,
            is_scrollable_x,
            is_scrollable_y,
            scrollable_size: content_size,
            scroll_offset: block_offset,
            children,
            line: shaper_block_node.line,
            column: shaper_block_node.column,
        }
    }

    fn get_content_size(children: &[ShaperBlockNode]) -> Size {
        let mut max_x: f32 = 0.0;
        let mut max_y: f32 = 0.0;

        for child in children {
            max_x = max_x.max(child.location.x + child.size.width);
            max_y = max_y.max(child.location.y + child.size.height);
        }

        Size { width: max_x, height: max_y }
    }
}

fn intersect_range(existing: (f32, f32), new: (f32, f32)) -> (f32, f32) {
    (existing.0.max(new.0), existing.1.min(new.1))
}