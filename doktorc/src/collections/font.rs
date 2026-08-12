use serde::{Serialize, Deserialize};

const FONT_NAMES: &[(&str, Font)] = &[
    ("arial", Font::Arial),
    ("helvetica", Font::Helvetica),
    ("verdana", Font::Verdana),
    ("tahoma", Font::Tahoma),
    ("trebuchet_ms", Font::TrebuchetMS),
    ("segoe_ui", Font::SegoeUI),
    ("georgia", Font::Georgia),
    ("times_new_roman", Font::TimesNewRoman),
    ("garamond", Font::Garamond),
    ("baskerville", Font::Baskerville),
    ("courier_new", Font::CourierNew),
    ("consolas", Font::Consolas),
    ("sans_serif", Font::SansSerif),
    ("serif", Font::Serif),
    ("monospace", Font::Monospace),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Font {
    Arial = 0,
    Helvetica = 1,
    Verdana = 2,
    Tahoma = 3,
    TrebuchetMS = 4,
    SegoeUI = 5,
    Georgia = 6,
    TimesNewRoman = 7,
    Garamond = 8,
    Baskerville = 9,
    CourierNew = 10,
    Consolas = 11,
    SansSerif = 12,
    Serif = 13,
    Monospace = 14,
}

impl Font {
    pub fn parse_font(value: &str) -> Option<Font> {
        FONT_NAMES.iter().find(|(name, _)| *name == value).map(|(_, font)| *font)
    }
}