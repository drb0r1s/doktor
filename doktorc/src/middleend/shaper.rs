use crate::frontend::resolved_ast::{ResolvedBlockNode, ResolvedDoktorNode};

use crate::middleend::layout::{Alignment, Direction, Layout, LayoutProperties, Location, Size};
use crate::middleend::drawable::{DrawableBlockNode, DrawableDoktorNode};

struct SizedBlockNode {
    block_type: String,
    tag: String,
    resolved_block_node: ResolvedBlockNode,
    layout_properties: LayoutProperties,
    size: Size,
    children: Vec<SizedBlockNode>,
    line: usize,
    column: usize,
}

pub struct Shaper {
    viewport_width: f32,
    viewport_height: f32,
}

impl Shaper {
    pub fn new(viewport_width: f32, viewport_height: f32) -> Self {
        Shaper {
            viewport_width,
            viewport_height,
        }
    }

    pub fn shape(&self, resolved_doktor_node: ResolvedDoktorNode) -> DrawableDoktorNode {
        // Pass 1: bottom-up sizing.
        let sized_children: Vec<SizedBlockNode> = resolved_doktor_node.children.into_iter().map(|resolved_block_node| self.size_block(resolved_block_node)).collect();

        // Pass 2: top-down location defining.
        // Setting default layout properties for the doktor node (root).
        let doktor_node_layout: LayoutProperties = LayoutProperties {
            layout: Layout::Simple,
            direction: Direction::Horizontal,
            alignment_x: Alignment::Start,
            alignment_y: Alignment::Start,
        };

        let children: Vec<DrawableBlockNode> = self.locate_children(
            &sized_children,
            doktor_node_layout,
            Location { x: 0.0, y: 0.0 },
            Size {
                width: self.viewport_width,
                height: self.viewport_height,
            },
        );

        DrawableDoktorNode { children }
    }

    // Pass 1: bottom-up sizing.

    fn size_block(&self, mut block: ResolvedBlockNode) -> SizedBlockNode {
        let children: Vec<ResolvedBlockNode> = std::mem::take(&mut block.children);
        let sized_children: Vec<SizedBlockNode> = children.into_iter().map(|child| self.size_block(child)).collect();

        let layout_properties: LayoutProperties = Self::resolve_layout_properties(&block.system_styles); // Assigning layout properties, the ones in system_styles or default.

        let size: Size = if sized_children.is_empty() {
            // Leaf: fixed size from its own width and height, or default.
            Size {
                width: block.system_styles.width.unwrap_or(crate::middleend::layout::DEFAULT_WIDTH),
                height: block.system_styles.height.unwrap_or(crate::middleend::layout::DEFAULT_HEIGHT),
            }
        } else {
            // Not a leaf: width and height of the block are ignored, instead the size is determined based on the block's children.
            match layout_properties.layout {
                Layout::Simple => match layout_properties.direction {
                    // width: sum of children widths
                    // height: max children height
                    Direction::Horizontal => Size {
                        width: sized_children.iter().map(|child| child.size.width).sum(),
                        height: sized_children.iter().map(|child| child.size.height).fold(0.0, f32::max),
                    },

                    // width: max children width
                    // height: sum of children heights
                    Direction::Vertical => Size {
                        width: sized_children.iter().map(|child| child.size.width).fold(0.0, f32::max),
                        height: sized_children.iter().map(|child| child.size.height).sum(),
                    },
                },

                Layout::Free => {
                    // width: maximal x-axis bounding box of a child.
                    // height: maximal y-axis bounding box of a child.
                    let mut max_x: f32 = 0.0;
                    let mut max_y: f32 = 0.0;

                    for child in &sized_children {
                        let position_x: f32 = child.resolved_block_node.system_styles.position_x.unwrap_or(0.0);
                        let position_y: f32 = child.resolved_block_node.system_styles.position_y.unwrap_or(0.0);

                        max_x = max_x.max(position_x + child.size.width);
                        max_y = max_y.max(position_y + child.size.height);
                    }

                    Size {
                        width: max_x,
                        height: max_y,
                    }
                }
            }
        };

        SizedBlockNode {
            block_type: block.block_type.clone(),
            tag: block.tag.clone(),
            layout_properties,
            size,
            line: block.line,
            column: block.column,
            resolved_block_node: block,
            children: sized_children,
        }
    }

    fn resolve_layout_properties(styles: &crate::frontend::resolved_ast::SystemStyles) -> LayoutProperties {
        let alignment_x: Alignment = styles.alignment_x.or(styles.alignment).unwrap_or(crate::middleend::layout::DEFAULT_ALIGNMENT);
        let alignment_y: Alignment = styles.alignment_y.or(styles.alignment).unwrap_or(crate::middleend::layout::DEFAULT_ALIGNMENT);

        LayoutProperties {
            layout: styles.layout.unwrap_or(crate::middleend::layout::DEFAULT_LAYOUT),
            direction: styles.direction.unwrap_or(crate::middleend::layout::DEFAULT_DIRECTION),
            alignment_x,
            alignment_y,
        }
    }

    // Pass 2: top-down location defining.

    fn locate_children(&self, children: &[SizedBlockNode], parent_layout: LayoutProperties, parent_location: Location, parent_size: Size) -> Vec<DrawableBlockNode> {
        match parent_layout.layout {
            Layout::Simple => {
                self.organize_location(children, parent_layout.direction, parent_location, parent_size)
            },

            Layout::Free => children.iter().map(|child| {
                let position_x: f32 = child.resolved_block_node.system_styles.position_x.unwrap_or(0.0);
                let position_y: f32 = child.resolved_block_node.system_styles.position_y.unwrap_or(0.0);

                let location: Location = Location {
                    x: parent_location.x + position_x,
                    y: parent_location.y + position_y,
                };

                self.get_drawable_block_node(child, location)
            }).collect()
        }
    }

    fn organize_location(&self, children: &[SizedBlockNode], parent_direction: Direction, parent_location: Location, parent_size: Size) -> Vec<DrawableBlockNode> {
        let mut result = Vec::with_capacity(children.len()); // We reserve children.len() positions in the vector.

        let mut breakable_cursor: f32 = 0.0;
        let mut scrollable_cursor: f32 = 0.0;

        let mut line_size: f32 = 0.0; // tallest/widest child in the current row/column

        for child in children {
            let (breakable_size, scrollable_size): (f32, f32) = match parent_direction {
                Direction::Horizontal => (child.size.width, child.size.height),
                Direction::Vertical => (child.size.height, child.size.width),
            };

            let breakable_bound: f32 = match parent_direction {
                Direction::Horizontal => self.viewport_width,
                Direction::Vertical => self.viewport_height,
            };

            if breakable_cursor > 0.0 && get_breakable_parent_location(parent_location, parent_direction) + breakable_cursor + breakable_size > breakable_bound {
                breakable_cursor = 0.0;
                scrollable_cursor += line_size;

                line_size = 0.0;
            }

            let location: Location = match parent_direction {
                Direction::Horizontal => Location {
                    x: parent_location.x + breakable_cursor,
                    y: parent_location.y + scrollable_cursor,
                },

                Direction::Vertical => Location {
                    x: parent_location.x + scrollable_cursor,
                    y: parent_location.y + breakable_cursor,
                },
            };

            result.push(self.get_drawable_block_node(child, location));

            breakable_cursor += breakable_size;
            line_size = line_size.max(scrollable_size);
        }

        result
    }

    fn get_drawable_block_node(&self, sized_block_node: &SizedBlockNode, location: Location) -> DrawableBlockNode {
        let children: Vec<DrawableBlockNode> = self.locate_children(&sized_block_node.children, sized_block_node.layout_properties, location, sized_block_node.size);

        DrawableBlockNode {
            block_type: sized_block_node.block_type.clone(),
            tag: sized_block_node.tag.clone(),
            system_attributes: sized_block_node.resolved_block_node.system_attributes.clone(),
            arbitrary_attributes: sized_block_node.resolved_block_node.arbitrary_attributes.clone(),
            system_styles: sized_block_node.resolved_block_node.system_styles.clone(),
            arbitrary_styles: sized_block_node.resolved_block_node.arbitrary_styles.clone(),
            layout_properties: sized_block_node.layout_properties,
            size: sized_block_node.size,
            location,
            children,
            line: sized_block_node.line,
            column: sized_block_node.column,
        }
    }
}

fn get_breakable_parent_location(location: Location, direction: Direction) -> f32 {
    match direction {
        Direction::Horizontal => location.x,
        Direction::Vertical => location.y,
    }
}