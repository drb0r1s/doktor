use crate::frontend::resolved_ast::RGB;

pub const DEFAULT_LAYOUT: Layout = Layout::Simple;
pub const DEFAULT_DIRECTION: Direction = Direction::Horizontal;
pub const DEFAULT_ALIGNMENT: Alignment = Alignment::Start;
pub const DEFAULT_WIDTH: f32 = 100.0;
pub const DEFAULT_HEIGHT: f32 = 100.0;
pub const DEFAULT_LOCATION: f32 = 0.0;
pub const DEFAULT_POSITION: f32 = 0.0;
pub const DEFAULT_FONT_SIZE: f32 = 16.0;
pub const DEFAULT_CONTENT_COLOR: RGB = RGB { r: 0, g: 0, b: 0 };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Simple,
    Free,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutProperties {
    pub layout: Layout,
    pub direction: Direction,
    pub alignment_x: Alignment,
    pub alignment_y: Alignment,
}

impl Default for LayoutProperties {
    fn default() -> Self {
        LayoutProperties {
            layout: DEFAULT_LAYOUT,
            direction: DEFAULT_DIRECTION,
            alignment_x: DEFAULT_ALIGNMENT,
            alignment_y: DEFAULT_ALIGNMENT,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Default for Size {
    fn default() -> Self {
        Size {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}

impl Default for Location {
    fn default() -> Self {
        Location {
            x: DEFAULT_LOCATION,
            y: DEFAULT_LOCATION,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
 
impl Default for Position {
    fn default() -> Self {
        Position {
            x: DEFAULT_POSITION,
            y: DEFAULT_POSITION,
        }
    }
}