use std::fmt;

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
            block_type: String::from("Doktor"),
            children: Vec::new(),
        }
    }
}

// Display adjustments.

impl fmt::Display for DoktorNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for child in &self.children {
            child.write_indented(f, 0)?;
        }

        Ok(())
    }
}

impl fmt::Display for BlockNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_indented(f, 0)
    }
}

impl BlockNode {
    fn write_indented(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        let padding: String = "  ".repeat(depth);

        write!(f, "{}[{}:{}] {}", padding, self.line, self.column, self.block_type)?;
        if !self.tag.is_empty() {
            write!(f, ": {}", self.tag)?;
        }
        writeln!(f)?;

        for attribute in &self.attributes {
            writeln!(
                f,
                "{}  | {} : {}",
                padding, attribute.name, attribute.value
            )?;
        }

        if !self.styles.is_empty() {
            let styles: Vec<String> = self
                .styles
                .iter()
                .map(|style: &Style| format!("{}: {}", style.name, style.value))
                .collect();
            writeln!(f, "{}  <{}>", padding, styles.join(", "))?;
        }

        for child in &self.children {
            child.write_indented(f, depth + 1)?;
        }

        Ok(())
    }
}