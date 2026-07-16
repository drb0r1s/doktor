use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    BlockStart, // [
    BlockEnd, // ]
    ChildrenStart, // {
    ChildrenEnd, // }
    StyleStart, // <
    StyleEnd, // >
    Colon, // :
    Comma, // ,
    Separator, // |
    Identifier, // block type, attribute name, style property name
    Value, // block tag, attribute value, style property value
    EndOfFile,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub content: String,
    pub line: usize,
    pub column: usize,
}

// Reimplementing println!("{}", token) in a more readable way.
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.text.is_empty() {
            write!(f, "[{}:{}] {:?}", self.line, self.column, self.token_type)
        }
        
        else {
            write!(
                f,
                "[{}:{}] {:?} '{}'",
                self.line, self.column, self.token_type, self.text
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenizerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

// Reimplementing println!("{}", tokenizer_error) in a more readable way.
impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tokenizer error at [{}:{}]: {}.",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for TokenizerError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Expect {
    Identifier,
    Value,
    Any,
}

pub struct Tokenizer {
    characters: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(source: &str) -> Self {
        Tokenizer {
            characters: source.chars().collect(), // Breaking a source string literal into a vector of characters.
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn is_end(&self) -> bool {
        self.position >= self.characters.len()
    }

    fn look_ahead(&self, offset: usize) -> char {
        let look_ahead_position: usize = self.position + offset;

        if look_ahead_position >= self.characters.len() {
            '\0'
        }
        
        else {
            self.characters[look_ahead_position]
        }
    }

    fn advance(&mut self) -> char {
        let character: char = self.characters[self.position];

        self.position += 1;

        if character == '\n' {
            self.line += 1;
            self.column = 1;
        }
        
        else {
            self.column += 1;
        }
        
        character
    }

    fn skip_whitespace(&mut self) {
        while !self.is_end() {
            let character: char = self.look_ahead(0);

            if character == ' ' || character == '\t' || character == '\r' || character == '\n' {
                self.advance();  
            }
            
            else {
                break;
            }
        }
    }

    fn is_identifier(character: char) -> bool {
        character.is_alphanumeric() || character == '_' || character == '-'
    }

    fn read_identifier(&mut self) -> Result<Token, TokenizerError> {
        let start_line: usize = self.line;
        let start_column: usize = self.column;

        let mut content = String::new();

        while !self.is_end() && Self::is_identifier(self.look_ahead(0)) {
            content.push(self.advance());
        }

        if content.is_empty() {
            return Err(TokenizerError {
                message: format!("Expected identifier, found '{}'.", self.look_ahead(0)),
                line: start_line,
                column: start_column,
            });
        }

        Ok(Token {
            token_type: TokenType::Identifier,
            content,
            line: start_line,
            column: start_column,
        })
    }

    fn read_value(&mut self, terminators: &str) -> Result<Token, TokenizerError> {
        let start_line: usize = self.line;
        let start_column: usize = self.column;

        let mut content = String::new();

        while !self.is_end() && !terminators.contains(self.look_ahead(0)) {
            content.push(self.advance());
        }

        let trimmed = content.trim_end().to_string();

        if trimmed.is_empty() {
            return Err(TokenizerError {
                message: format!("Expected value before '{}'.", self.look_ahead(0)),
                line: start_line,
                column: start_column,
            });
        }

        Ok(Token {
            token_type: TokenType::Value,
            content: trimmed,
            line: start_line,
            column: start_column,
        })
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, TokenizerError> {
        let mut tokens = Vec::new();
        let mut expect = Expect::Any;

        loop {
            self.skip_whitespace();

            if self.is_end() {
                tokens.push(Token {
                    token_type: TokenType::EndOfFile,
                    content: String::new(),
                    line: self.line,
                    column: self.column,
                });

                break;
            }

            let start_line: usize = self.line;
            let start_column: usize = self.column;

            let character: char = self.look_ahead(0);

            match character {
                '[' => {
                    self.advance();

                    tokens.push(Token {
                        token_type: TokenType::BlockStart,
                        content: "[".to_string(),
                        line: start_line,
                        column: start_column,
                    });

                    expect = Expect::Identifier;
                }

                ']' => {
                    self.advance();

                    tokens.push(Token {
                        token_type: TokenType::BlockEnd,
                        content: "]".to_string(),
                        line: start_line,
                        column: start_column,
                    });

                    expect = Expect::Any;
                }

                '{' => {
                    self.advance();

                    tokens.push(Token {
                        token_type: TokenType::ChildrenStart,
                        content: "{".to_string(),
                        line: start_line,
                        column: start_column,
                    });

                    expect = Expect::Identifier;
                }

                '}' => {
                    self.advance();

                    tokens.push(Token {
                        token_type: TokenType::ChildrenEnd,
                        content: "}".to_string(),
                        line: start_line,
                        column: start_column,
                    });

                    expect = Expect::Any;
                }

                '<' => {
                    self.advance();

                    tokens.push(Token {
                        token_type: TokenType::StyleStart,
                        content: "<".to_string(),
                        line: start_line,
                        column: start_column,
                    });

                    expect = Expect::Identifier;
                }

                '>' => {
                    self.advance();

                    tokens.push(Token {
                        token_type: TokenType::StyleEnd,
                        content: ">".to_string(),
                        line: start_line,
                        column: start_column,
                    });
                    
                    expect = Expect::Any;
                }

                ':' => {
                    self.advance();

                    tokens.push(Token {
                        token_type: TokenType::Colon,
                        content: ":".to_string(),
                        line: start_line,
                        column: start_column,
                    });

                    expect = Expect::Value;
                }

                ',' => {
                    self.advance();

                    tokens.push(Token {
                        token_type: TokenType::Comma,
                        content: ",".to_string(),
                        line: start_line,
                        column: start_column,
                    });

                    expect = Expect::Identifier;
                }

                '|' => {
                    self.advance();

                    tokens.push(Token {
                        token_type: TokenType::Separator,
                        content: "|".to_string(),
                        line: start_line,
                        column: start_column,
                    });

                    expect = Expect::Identifier;
                }

                _ => {
                    let token = if expect == Expect::Value {
                        self.read_value("|],}")?
                    } else {
                        self.read_identifier()?
                    };

                    expect = Expect::Any;

                    tokens.push(token);
                }
            }
        }

        Ok(tokens)
    }
}