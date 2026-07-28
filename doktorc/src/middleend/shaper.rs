use crate::frontend::resolver_ast::{SystemStyles, Alignment, Direction, Layout, ResolverBlockNode, ResolverDoktorNode};

use crate::middleend::shaper_ast::{Size, Location, TextMeasurement, ShaperBlockNode, ShaperDoktorNode};

struct SizedResolverBlockNode {
    resolver_block_node: ResolverBlockNode,
    size: Size,
    children: Vec<SizedResolverBlockNode>,
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

    pub fn shape(&self, resolver_doktor_node: ResolverDoktorNode, text_measurements: &[TextMeasurement]) -> ShaperDoktorNode {
        // Pass 1: bottom-up sizing.
        let mut path: Vec<usize> = Vec::new();

        let sized_children: Vec<SizedResolverBlockNode> = resolver_doktor_node.children.into_iter().enumerate().map(|(index, resolver_block_node)| {
            path.push(index);
            let sized = self.size_block(resolver_block_node, text_measurements, &mut path);
            path.pop();

            sized
        }).collect();

        // Pass 2: top-down location defining.
        // Setting default layout properties for the doktor node (root).

        let resolver_doktor_node_system_styles: SystemStyles = SystemStyles::default(false);

        let children: Vec<ShaperBlockNode> = self.locate_children(
            &sized_children,
            &resolver_doktor_node_system_styles,
            Location { x: 0.0, y: 0.0 },
            Size {
                width: self.viewport_width,
                height: self.viewport_height,
            },
        );

        ShaperDoktorNode { children }
    }

    // Pass 1: bottom-up sizing.

    fn size_block(&self, mut block: ResolverBlockNode, text_measurements: &[TextMeasurement], path: &mut Vec<usize>) -> SizedResolverBlockNode {
        let children: Vec<ResolverBlockNode> = std::mem::take(&mut block.children);

        let sized_children: Vec<SizedResolverBlockNode> = children.into_iter().enumerate().map(|(index, child)| {
            path.push(index);
            let sized = self.size_block(child, text_measurements, path);
            path.pop();

            sized
        }).collect();

        let size: Size = if sized_children.is_empty() {
            // Leaf: fixed size from its own width and height, or default.
            // If a block is of type Text, then JS measured width and height are used.
            if block.block_type == "Text" {
                match text_measurements.iter().find(|text_measurement| &text_measurement.path == path) {
                    Some(measurement) => Size {
                        width: measurement.width,
                        height: measurement.height,
                    },

                    None => Size {
                        width: block.system_styles.width,
                        height: block.system_styles.height,
                    },
                }
            }
            
            else {
                Size {
                    width: block.system_styles.width,
                    height: block.system_styles.height,
                }
            }
        } else {
            // Not a leaf: width and height of the block are ignored (unless it is bigger than the minimal value), instead the size is determined based on the block's children.
            let style_width: f32 = block.system_styles.width;
            let style_height: f32 = block.system_styles.height;
            
            match block.system_styles.layout {
                Layout::Simple => match block.system_styles.direction {
                    // width: sum of children widths
                    // height: max children height
                    Direction::Horizontal => {
                        let minimal_width: f32 = sized_children.iter().map(|child| child.size.width).sum();
                        let minimal_height: f32 = sized_children.iter().map(|child| child.size.height).fold(0.0, f32::max);
                        
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
                        let position_x: f32 = child.resolver_block_node.system_styles.position_x.unwrap_or(0.0);
                        let position_y: f32 = child.resolver_block_node.system_styles.position_y.unwrap_or(0.0);

                        max_x = max_x.max(position_x + child.size.width);
                        max_y = max_y.max(position_y + child.size.height);
                    }

                    Size {
                        width: max_x.max(style_width),
                        height: max_y.max(style_height),
                    }
                }
            }
        };

        SizedResolverBlockNode {
            resolver_block_node: block,
            size,
            children: sized_children,
        }
    }

    // Pass 2: top-down location defining.

    fn locate_children(&self, children: &Vec<SizedResolverBlockNode>, parent_styles: &SystemStyles, parent_location: Location, parent_size: Size) -> Vec<ShaperBlockNode> {
        let (inset_location, inset_size): (Location, Size) = Self::apply_border_inset(parent_styles, parent_location, parent_size);
        
        match parent_styles.layout {
            Layout::Simple => {
                self.organize_location(children, parent_styles, inset_location, inset_size)
            },

            Layout::Free => children.iter().map(|child| {
                let position_x: f32 = child.resolver_block_node.system_styles.position_x.or(Some(child.resolver_block_node.system_styles.position)).unwrap_or(0.0);
                let position_y: f32 = child.resolver_block_node.system_styles.position_y.or(Some(child.resolver_block_node.system_styles.position)).unwrap_or(0.0);

                let location: Location = Location {
                    x: inset_location.x + position_x,
                    y: inset_location.y + position_y,
                };

                self.get_shaper_block_node(child, location, parent_styles.opacity)
            }).collect()
        }
    }

    fn organize_location(&self, children: &[SizedResolverBlockNode], parent_styles: &SystemStyles, parent_location: Location, parent_size: Size) -> Vec<ShaperBlockNode> {
        let parent_direction = parent_styles.direction;

        let breakable_bound: f32 = match parent_direction {
            Direction::Horizontal => parent_size.width,
            Direction::Vertical => parent_size.height,
        };

        let parent_scrollable_size: f32 = match parent_direction {
            Direction::Horizontal => parent_size.height,
            Direction::Vertical => parent_size.width,
        };

        let mut lines: Vec<Vec<&SizedResolverBlockNode>> = Vec::new();
        let mut current_line: Vec<&SizedResolverBlockNode> = Vec::new();
        
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
            Direction::Horizontal => parent_styles.get_unambiguous_alignment("x"),
            Direction::Vertical => parent_styles.get_unambiguous_alignment("y"),
        };

        let scrollable_alignment: Alignment = match parent_direction {
            Direction::Horizontal => parent_styles.get_unambiguous_alignment("y"),
            Direction::Vertical => parent_styles.get_unambiguous_alignment("x"),
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

                let scrollable_leftover: f32 = (parent_scrollable_size - scrollable_size).max(0.0);

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

                result.push(self.get_shaper_block_node(child, location, parent_styles.opacity));
                breakable_cursor += breakable_size;
            }

            scrollable_cursor += line_scrollable_size;
        }

        result
    }

    fn apply_border_inset(parent_styles: &SystemStyles, parent_location: Location, parent_size: Size) -> (Location, Size) {
        let border_size: f32 = parent_styles.border_size;

        if border_size <= 0.0 {
            return (parent_location, parent_size);
        }

        let inset_location = Location {
            x: parent_location.x + border_size,
            y: parent_location.y + border_size,
        };

        let inset_size = Size {
            width: (parent_size.width - border_size * 2.0).max(0.0),
            height: (parent_size.height - border_size * 2.0).max(0.0),
        };

        (inset_location, inset_size)
    }

    fn get_shaper_block_node(&self, sized_resolver_block_node: &SizedResolverBlockNode, location: Location, inherited_opacity: f32) -> ShaperBlockNode {
        let mut system_styles = sized_resolver_block_node.resolver_block_node.system_styles.clone();
        system_styles.opacity *= inherited_opacity;

        let children: Vec<ShaperBlockNode> = self.locate_children(&sized_resolver_block_node.children, &system_styles, location, sized_resolver_block_node.size);

        ShaperBlockNode {
            block_type: sized_resolver_block_node.resolver_block_node.block_type.clone(),
            tag: sized_resolver_block_node.resolver_block_node.tag.clone(),
            system_attributes: sized_resolver_block_node.resolver_block_node.system_attributes.clone(),
            arbitrary_attributes: sized_resolver_block_node.resolver_block_node.arbitrary_attributes.clone(),
            system_styles,
            arbitrary_styles: sized_resolver_block_node.resolver_block_node.arbitrary_styles.clone(),
            size: sized_resolver_block_node.size,
            location,
            children,
            line: sized_resolver_block_node.resolver_block_node.line,
            column: sized_resolver_block_node.resolver_block_node.column,
        }
    }
}

fn get_breakable_parent_location(location: Location, direction: Direction) -> f32 {
    match direction {
        Direction::Horizontal => location.x,
        Direction::Vertical => location.y,
    }
}