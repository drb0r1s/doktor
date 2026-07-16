#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub name: String,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockNode {
    pub block_type: String,
    pub tag: String,
    pub attributes: Vec<Attribute>,
    pub styles: Vec<Style>,
    pub children: Vec<BlockNode>,
    pub line: usize,
    pub column: usize,
}

// Root of the AST.
#[derive(Debug, Clone, PartialEq)]
pub struct DoktorNode {
    pub block_type: String,
    pub children: Vec<BlockNode>,
}

impl DoktorNode {
    pub fn generate() -> Self {
        DoktorNode {
            block_type: "Doktor",
            children: Vec::new(),
        }
    }
}