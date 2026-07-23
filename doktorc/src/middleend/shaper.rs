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
            // Not a leaf: width and height of the block are ignored (unless it is bigger than the minimal value), instead the size is determined based on the block's children.
            match layout_properties.layout {
                Layout::Simple => match layout_properties.direction {
                    // width: sum of children widths
                    // height: max children height
                    Direction::Horizontal => {
                        let minimal_width: f32 = sized_children.iter().map(|child| child.size.width).sum();
                        let minimal_height: f32 = sized_children.iter().map(|child| child.size.height).fold(0.0, f32::max);

                        let style_width: f32 = block.system_styles.width.unwrap_or(crate::middleend::layout::DEFAULT_WIDTH);
                        let style_height: f32 = block.system_styles.height.unwrap_or(crate::middleend::layout::DEFAULT_HEIGHT);
                        
                        Size {
                            width: minimal_width.max(style_width),
                            height: minimal_height.max(style_height),
                        }
                    },

                    // width: max children width
                    // height: sum of children heights
                    Direction::Vertical => {
                        let minimal_width: f32 = sized_children.iter().map(|child| child.size.width).fold(0.0, f32::max);
                        let minimal_height: f32 = sized_children.iter().map(|child| child.size.height).sum();

                        let style_width: f32 = block.system_styles.width.unwrap_or(crate::middleend::layout::DEFAULT_WIDTH);
                        let style_height: f32 = block.system_styles.height.unwrap_or(crate::middleend::layout::DEFAULT_HEIGHT);
                        
                        Size {
                            width: minimal_width.max(style_width),
                            height: minimal_height.max(style_height),
                        }
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
                self.organize_location(children, parent_layout, parent_location, parent_size)
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

    fn organize_location(&self, children: &[SizedBlockNode], parent_layout: LayoutProperties, parent_location: Location, parent_size: Size) -> Vec<DrawableBlockNode> {
        let parent_direction = parent_layout.direction;

        let breakable_bound: f32 = match parent_direction {
            Direction::Horizontal => parent_size.width,
            Direction::Vertical => parent_size.height,
        };

        let mut lines: Vec<Vec<&SizedBlockNode>> = Vec::new();
        let mut current_line: Vec<&SizedBlockNode> = Vec::new();
        
        let mut breakable_cursor: f32 = 0.0;

        for child in children {
            let breakable_size: f32 = match parent_direction {
                Direction::Horizontal => child.size.width,
                Direction::Vertical => child.size.height,
            };

            let parent_breakable_location: f32 = get_breakable_parent_location(parent_location, parent_direction);

            if breakable_cursor > 0.0 && parent_breakable_location + breakable_cursor + breakable_size > breakable_bound {
                lines.push(std::mem::take(&mut current_line));
                breakable_cursor = 0.0;
            }

            current_line.push(child);
            breakable_cursor += breakable_size;
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        let breakable_alignment: Alignment = match parent_direction {
            Direction::Horizontal => parent_layout.alignment_x,
            Direction::Vertical => parent_layout.alignment_y,
        };

        let scrollable_alignment: Alignment = match parent_direction {
            Direction::Horizontal => parent_layout.alignment_y,
            Direction::Vertical => parent_layout.alignment_x,
        };

        let mut result = Vec::with_capacity(children.len());
        let mut scrollable_cursor: f32 = 0.0;

        for line in &lines {
            let line_breakable_size: f32 = line.iter().map(|child| match parent_direction {
                Direction::Horizontal => child.size.width,
                Direction::Vertical => child.size.height,
            }).sum();

            let line_scrollable_size: f32 = line.iter().map(|child| match parent_direction {
                Direction::Horizontal => child.size.height,
                Direction::Vertical => child.size.width,
            }).fold(0.0, f32::max);

            let line_breakable_leftover: f32 = (breakable_bound - get_breakable_parent_location(parent_location, parent_direction) - line_breakable_size).max(0.0);

            let line_breakable_start_offset: f32 = match breakable_alignment {
                Alignment::Start => 0.0,
                Alignment::Center => line_breakable_leftover / 2.0,
                Alignment::End => line_breakable_leftover,
            };

            let mut breakable_cursor: f32 = line_breakable_start_offset;

            for child in line {
                let (breakable_size, scrollable_size): (f32, f32) = match parent_direction {
                    Direction::Horizontal => (child.size.width, child.size.height),
                    Direction::Vertical => (child.size.height, child.size.width),
                };

                let scrollable_leftover: f32 = (line_scrollable_size - scrollable_size).max(0.0);

                let scrollable_offset: f32 = match scrollable_alignment {
                    Alignment::Start => 0.0,
                    Alignment::Center => scrollable_leftover / 2.0,
                    Alignment::End => scrollable_leftover,
                };

                let location: Location = match parent_direction {
                    Direction::Horizontal => Location {
                        x: parent_location.x + breakable_cursor,
                        y: parent_location.y + scrollable_cursor + scrollable_offset,
                    },

                    Direction::Vertical => Location {
                        x: parent_location.x + scrollable_cursor + scrollable_offset,
                        y: parent_location.y + breakable_cursor,
                    },
                };

                result.push(self.get_drawable_block_node(child, location));
                breakable_cursor += breakable_size;
            }

            scrollable_cursor += line_scrollable_size;
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