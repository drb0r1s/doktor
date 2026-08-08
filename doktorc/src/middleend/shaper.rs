use crate::frontend::resolver_ast::{SystemStyles, Alignment, Direction, Layout, Overflow, ResolverBlockNode, ResolverDoktorNode};

use crate::middleend::shaper_ast::{Size, Location, Clip, TextMeasurement, ImageMeasurement, ShaperBlockNode, ShaperDoktorNode};

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

    pub fn shape(&self, resolver_doktor_node: ResolverDoktorNode, text_measurements: &[TextMeasurement], image_measurements: &[ImageMeasurement]) -> ShaperDoktorNode {
        // Pass 1: bottom-up sizing.
        let mut path: Vec<usize> = Vec::new();

        let mut sized_children: Vec<SizedResolverBlockNode> = resolver_doktor_node.children.into_iter().enumerate().map(|(index, resolver_block_node)| {
            path.push(index);
            let sized = self.size_block(resolver_block_node, text_measurements, image_measurements, &mut path);
            path.pop();

            sized
        }).collect();

        // Pass 1.5: top-down percentage dimensions resolution.

        let viewport_size: Size = Size {
            width: self.viewport_width,
            height: self.viewport_height,
        };

        for child in &mut sized_children {
            Self::resolve_dimension_percentages(child, viewport_size);
        }

        // Pass 2: top-down location defining.
        // Setting default layout properties for the doktor node (root).

        let resolver_doktor_node_system_styles: SystemStyles = SystemStyles::default(false);

        let children: Vec<ShaperBlockNode> = self.locate_children(
            &sized_children,
            &resolver_doktor_node_system_styles,
            Location { x: 0.0, y: 0.0 },
            viewport_size,
            Clip {
                x: (0.0, self.viewport_width),
                y: (0.0, self.viewport_height),
            },
        );

        ShaperDoktorNode { children }
    }

    // Pass 1: bottom-up sizing.

    fn size_block(&self, mut block: ResolverBlockNode, text_measurements: &[TextMeasurement], image_measurements: &[ImageMeasurement], path: &mut Vec<usize>) -> SizedResolverBlockNode {
        let children: Vec<ResolverBlockNode> = std::mem::take(&mut block.children);

        let sized_children: Vec<SizedResolverBlockNode> = children.into_iter().enumerate().map(|(index, child)| {
            path.push(index);
            let sized = self.size_block(child, text_measurements, image_measurements, path);
            path.pop();

            sized
        }).collect();

        let mut size: Size = if sized_children.is_empty() {
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

            else if block.block_type == "Image" {
                match image_measurements.iter().find(|image_measurement| &image_measurement.path == path) {
                    Some(measurement) => Size {
                        width: if block.system_styles.width > 0.0 { block.system_styles.width } else { measurement.width },
                        height: if block.system_styles.height > 0.0 { block.system_styles.height } else { measurement.height },
                    },

                    None => Size { width: block.system_styles.width, height: block.system_styles.height },
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

            let lock_width: bool = block.system_styles.get_unambiguous_lock_dimensions("width");
            let lock_height: bool = block.system_styles.get_unambiguous_lock_dimensions("height");
            
            let computed_dimensions: Size = match block.system_styles.layout {
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
            };

            Size {
                width: if lock_width { style_width } else { computed_dimensions.width },
                height: if lock_height { style_height } else { computed_dimensions.height },
            }
        };

        // We have to adjust block's size to the inset, in order to achieve correct sizing of the block and prevent overflow.
        let padding_top: f32 = block.system_styles.get_unambiguous_spacing("padding", "top");
        let padding_bottom: f32 = block.system_styles.get_unambiguous_spacing("padding", "bottom");
        let padding_left: f32 = block.system_styles.get_unambiguous_spacing("padding", "left");
        let padding_right: f32 = block.system_styles.get_unambiguous_spacing("padding", "right");

        let inset_x: f32 = padding_left + padding_right + block.system_styles.border_size * 2.0;
        let inset_y: f32 = padding_top + padding_bottom + block.system_styles.border_size * 2.0;

        if inset_x > 0.0 || inset_y > 0.0 {
            size.width = (size.width + inset_x).max(0.0);
            size.height = (size.height + inset_y).max(0.0);

            block.system_styles.width = size.width;
            block.system_styles.height = size.height;
        }

        SizedResolverBlockNode {
            resolver_block_node: block,
            size,
            children: sized_children,
        }
    }

    // Pass 1.5: top-down percentage dimensions resolution.

    fn resolve_dimension_percentages(block: &mut SizedResolverBlockNode, parent_size: Size) {
        if let Some(percentage) = block.resolver_block_node.system_styles.width_percent {
            block.size.width = parent_size.width * percentage;
        }

        if let Some(percentage) = block.resolver_block_node.system_styles.height_percent {
            block.size.height = parent_size.height * percentage;
        }

        let inset_size: Size = Self::get_inset_size(&block.resolver_block_node.system_styles, block.size);

        for child in &mut block.children {
            Self::resolve_dimension_percentages(child, inset_size);
        }
    }

    fn get_inset_size(styles: &SystemStyles, total_size: Size) -> Size {
        let padding_top: f32 = styles.get_unambiguous_spacing("padding", "top");
        let padding_bottom: f32 = styles.get_unambiguous_spacing("padding", "bottom");
        let padding_left: f32 = styles.get_unambiguous_spacing("padding", "left");
        let padding_right: f32 = styles.get_unambiguous_spacing("padding", "right");

        let border_top: f32 = styles.get_unambiguous_border_size("top");
        let border_bottom: f32 = styles.get_unambiguous_border_size("bottom");
        let border_left: f32 = styles.get_unambiguous_border_size("left");
        let border_right: f32 = styles.get_unambiguous_border_size("right");

        Size {
            width: (total_size.width - padding_left - padding_right - border_left - border_right).max(0.0),
            height: (total_size.height - padding_top - padding_bottom - border_top - border_bottom).max(0.0),
        }
    }

    // Pass 2: top-down location defining.

    fn locate_children(&self, children: &Vec<SizedResolverBlockNode>, parent_styles: &SystemStyles, parent_location: Location, parent_size: Size, parent_clip: Clip) -> Vec<ShaperBlockNode> {
        let (inset_location, inset_size): (Location, Size) = Self::apply_inset(parent_styles, parent_location, parent_size);
        
        match parent_styles.layout {
            Layout::Simple => {
                self.organize_location(children, parent_styles, inset_location, inset_size, parent_clip)
            },

            Layout::Free => children.iter().map(|child| {
                let position_x: f32 = child.resolver_block_node.system_styles.position_x.or(Some(child.resolver_block_node.system_styles.position)).unwrap_or(0.0);
                let position_y: f32 = child.resolver_block_node.system_styles.position_y.or(Some(child.resolver_block_node.system_styles.position)).unwrap_or(0.0);

                let location: Location = Location {
                    x: inset_location.x + position_x,
                    y: inset_location.y + position_y,
                };

                self.get_shaper_block_node(child, location, parent_size, parent_clip, parent_styles.opacity)
            }).collect()
        }
    }

    fn organize_location(&self, children: &[SizedResolverBlockNode], parent_styles: &SystemStyles, parent_location: Location, parent_size: Size, parent_clip: Clip) -> Vec<ShaperBlockNode> {
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

            if breakable_cursor > 0.0 && breakable_cursor + breakable_size > breakable_bound {
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

        let line_count: f32 = lines.len() as f32;

        for line in &lines {
            let line_breakable_size: f32 = line.iter().map(|child| {
                let (margin_start, margin_end): (f32, f32) = Self::get_breakable_margin(&child.resolver_block_node.system_styles, parent_direction);

                (match parent_direction {
                    Direction::Horizontal => child.size.width,
                    Direction::Vertical => child.size.height,
                }) + margin_start + margin_end
            }).sum();

            let line_scrollable_size: f32 = line.iter().map(|child| {
                let (margin_start, margin_end): (f32, f32) = Self::get_scrollable_margin(&child.resolver_block_node.system_styles, parent_direction);

                (match parent_direction {
                    Direction::Horizontal => child.size.height,
                    Direction::Vertical => child.size.width,
                }) + margin_start + margin_end
            }).fold(0.0, f32::max);

            let line_breakable_leftover: f32 = (breakable_bound - line_breakable_size).max(0.0);

            let line_breakable_start_offset: f32 = match breakable_alignment {
                Alignment::Start => 0.0,
                Alignment::Center => line_breakable_leftover / 2.0,
                Alignment::End => line_breakable_leftover,
            };

            let mut breakable_cursor: f32 = line_breakable_start_offset;

            for child in line {
                let (breakable_margin_start, breakable_margin_end): (f32, f32) = Self::get_breakable_margin(&child.resolver_block_node.system_styles, parent_direction);
                let (scrollable_margin_start, scrollable_margin_end): (f32, f32) = Self::get_scrollable_margin(&child.resolver_block_node.system_styles, parent_direction);

                let (breakable_size, scrollable_size): (f32, f32) = match parent_direction {
                    Direction::Horizontal => (child.size.width + breakable_margin_start + breakable_margin_end, child.size.height + scrollable_margin_start + scrollable_margin_end),
                    Direction::Vertical => (child.size.height + breakable_margin_start + breakable_margin_end, child.size.width + scrollable_margin_start + scrollable_margin_end),
                };

                let scrollable_leftover: f32 = (parent_scrollable_size - scrollable_size).max(0.0);
                let scrollable_adjustment: f32 = line_scrollable_size * (line_count - 1.0); // Center and End have to be adjusted based on the number of lines blocks create.

                let scrollable_offset: f32 = match scrollable_alignment {
                    Alignment::Start => 0.0,
                    Alignment::Center => (scrollable_leftover - scrollable_adjustment) / 2.0,
                    Alignment::End => scrollable_leftover - scrollable_adjustment,
                };

                let location: Location = match parent_direction {
                    Direction::Horizontal => Location {
                        x: parent_location.x + breakable_cursor + breakable_margin_start,
                        y: parent_location.y + scrollable_cursor + scrollable_offset + scrollable_margin_start,
                    },

                    Direction::Vertical => Location {
                        x: parent_location.x + scrollable_cursor + scrollable_offset + scrollable_margin_start,
                        y: parent_location.y + breakable_cursor + breakable_margin_start,
                    },
                };

                result.push(self.get_shaper_block_node(child, location, parent_size, parent_clip.clone(), parent_styles.opacity));
                breakable_cursor += breakable_size;
            }

            scrollable_cursor += line_scrollable_size;
        }

        result
    }

    // Inset is currently affected by border_size and padding.
    fn apply_inset(parent_styles: &SystemStyles, parent_location: Location, parent_size: Size) -> (Location, Size) {
        let padding_top: f32 = parent_styles.get_unambiguous_spacing("padding", "top");
        let padding_bottom: f32 = parent_styles.get_unambiguous_spacing("padding", "bottom");
        let padding_left: f32 = parent_styles.get_unambiguous_spacing("padding", "left");
        let padding_right: f32 = parent_styles.get_unambiguous_spacing("padding", "right");

        let border_top: f32 = parent_styles.get_unambiguous_border_size("top");
        let border_bottom: f32 = parent_styles.get_unambiguous_border_size("bottom");
        let border_left: f32 = parent_styles.get_unambiguous_border_size("left");
        let border_right: f32 = parent_styles.get_unambiguous_border_size("right");

        let inset_location = Location {
            x: parent_location.x + padding_left + border_left,
            y: parent_location.y + padding_top + border_top,
        };

        let inset_size = Size {
            width: (parent_size.width - padding_left - padding_right - border_left - border_right).max(0.0),
            height: (parent_size.height - padding_top - padding_bottom - border_top - border_bottom).max(0.0),
        };

        (inset_location, inset_size)
    }

    fn get_shaper_block_node(&self, sized_resolver_block_node: &SizedResolverBlockNode, parent_location: Location, parent_size: Size, inherited_clip: Clip, inherited_opacity: f32) -> ShaperBlockNode {
        let mut system_styles: SystemStyles = sized_resolver_block_node.resolver_block_node.system_styles.clone();
        system_styles.opacity *= inherited_opacity;

        let size: Size = sized_resolver_block_node.size;

        let overflow_x: Overflow = system_styles.get_unambiguous_overflow("x");
        let overflow_y: Overflow = system_styles.get_unambiguous_overflow("y");

        let clip: Clip = Clip {
            x: if overflow_x == Overflow::False || overflow_x == Overflow::Scroll {
                intersect_range(inherited_clip.x, (parent_location.x, parent_location.x + size.width))
            } else {
                inherited_clip.x
            },

            y: if overflow_y == Overflow::False || overflow_y == Overflow::Scroll {
                intersect_range(inherited_clip.y, (parent_location.y, parent_location.y + size.height))
            } else {
                inherited_clip.y
            },
        };

        let children: Vec<ShaperBlockNode> = self.locate_children(&sized_resolver_block_node.children, &system_styles, parent_location, sized_resolver_block_node.size, clip.clone());

        ShaperBlockNode {
            id: sized_resolver_block_node.resolver_block_node.id,
            block_type: sized_resolver_block_node.resolver_block_node.block_type.clone(),
            tag: sized_resolver_block_node.resolver_block_node.tag.clone(),
            system_attributes: sized_resolver_block_node.resolver_block_node.system_attributes.clone(),
            arbitrary_attributes: sized_resolver_block_node.resolver_block_node.arbitrary_attributes.clone(),
            system_styles,
            arbitrary_styles: sized_resolver_block_node.resolver_block_node.arbitrary_styles.clone(),
            size,
            location: parent_location,
            clip,
            children,
            line: sized_resolver_block_node.resolver_block_node.line,
            column: sized_resolver_block_node.resolver_block_node.column,
        }
    }

    fn get_breakable_margin(styles: &SystemStyles, direction: Direction) -> (f32, f32) {
        match direction {
            Direction::Horizontal => (styles.get_unambiguous_spacing("margin", "left"), styles.get_unambiguous_spacing("margin", "right")),
            Direction::Vertical => (styles.get_unambiguous_spacing("margin", "top"), styles.get_unambiguous_spacing("margin", "bottom")),
        }
    }

    fn get_scrollable_margin(styles: &SystemStyles, direction: Direction) -> (f32, f32) {
        match direction {
            Direction::Horizontal => (styles.get_unambiguous_spacing("margin", "top"), styles.get_unambiguous_spacing("margin", "bottom")),
            Direction::Vertical => (styles.get_unambiguous_spacing("margin", "left"), styles.get_unambiguous_spacing("margin", "right")),
        }
    }
}

fn intersect_range(existing: (f32, f32), new: (f32, f32)) -> (f32, f32) {
    (existing.0.max(new.0), existing.1.min(new.1))
}