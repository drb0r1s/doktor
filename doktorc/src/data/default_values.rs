use crate::frontend::resolver_ast::{RGB, Layout, Direction, Alignment, Font};

pub const DEFAULT_LAYOUT: Layout = Layout::Simple;
pub const DEFAULT_DIRECTION: Direction = Direction::Horizontal;
pub const DEFAULT_ALIGNMENT: Alignment = Alignment::Start;

pub const DEFAULT_WIDTH: f32 = 100.0;
pub const DEFAULT_HEIGHT: f32 = 100.0;

pub const DEFAULT_LOCATION: f32 = 0.0;
pub const DEFAULT_POSITION: f32 = 0.0;

pub const DEFAULT_CONTENT_COLOR: RGB = RGB { r: 0, g: 0, b: 0 };
pub const DEFAULT_CONTENT_SIZE: f32 = 16.0;
pub const DEFAULT_CONTENT_FONT: Font = Font::Arial;

pub const DEFAULT_BACKGROUND_COLOR: RGB = RGB { r: 0, g: 0, b: 0 };