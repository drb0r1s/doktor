use std::fmt;

use crate::frontend::ast::{Attribute, Style, BlockNode, DoktorNode};
use crate::frontend::tokenizer::{TokenType, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

// Reimplementing println!("{}", parser_error) in a more readable way.
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parser error at [{}:{}]: {}.",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for ParserError {}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) -> Token {
        let token: Token = self.tokens[self.position].clone();

        if !matches!(token.token_type, TokenType::EndOfFile) {
            self.position += 1;
        }

        token
    }

    fn is_current_token_type(&self, token_type: &TokenType) -> bool {
        &self.current_token().token_type == token_type
    }

    fn expect(&mut self, token_type: TokenType, context: &str) -> Result<Token, ParserError> {
        if self.is_current_token_type(&token_type) {
            Ok(self.advance())
        }
        
        else {
            let found_token: &Token = self.current_token();

            Err(ParserError {
                message: format!(
                    "Expected {:?} {}, found {:?} '{}'.",
                    token_type, context, found_token.token_type, found_token.content
                ),
                line: found_token.line,
                column: found_token.column,
            })
        }
    }

    pub fn parse(mut self) -> Result<DoktorNode, ParserError> {
        let mut doktor_node: DoktorNode = DoktorNode::generate();

        while !self.is_current_token_type(&TokenType::EndOfFile) {
            let block: BlockNode = self.parse_block()?;
            doktor_node.children.push(block);
        }

        Ok(doktor_node)
    }

    fn parse_block(&mut self) -> Result<BlockNode, ParserError> {
        let block_start_token: Token = self.expect(TokenType::BlockStart, "to start a block")?;
        let identifier_token: Token = self.expect(TokenType::Identifier, "for block type")?;

        let mut block_node = BlockNode {
            block_type: identifier_token.content,
            tag: String::new(),
            attributes: Vec::new(),
            styles: Vec::new(),
            children: Vec::new(),
            line: block_start_token.line,
            column: block_start_token.column,
        };

        // Block tag check.
        if self.is_current_token_type(&TokenType::Colon) {
            self.advance();

            let block_tag_token: Token = self.expect(TokenType::Value, "for block tag")?;
            block_node.tag = block_tag_token.content;
        }

        // Attributes and Styles processing.
        while self.is_current_token_type(&TokenType::Separator) {
            self.advance();

            if self.is_current_token_type(&TokenType::StyleStart) {
                block_node.styles = self.parse_styles()?;
            }
            
            else {
                let attribute: Attribute = self.parse_attribute()?;
                block_node.attributes.push(attribute);
            }
        }

        self.expect(TokenType::BlockEnd, "to close a block")?;

        // Block children processing.
        if self.is_current_token_type(&TokenType::ChildrenStart) {
            self.advance();

            while !self.is_current_token_type(&TokenType::ChildrenEnd) {
                let child_block_node: BlockNode = self.parse_block()?;
                block_node.children.push(child_block_node);
            }

            self.expect(TokenType::ChildrenEnd, "to close a children block")?;
        }

        Ok(block_node)
    }

    fn parse_attribute(&mut self) -> Result<Attribute, ParserError> {
        let name_token: Token = self.expect(TokenType::Identifier, "for attribute name")?;
        self.expect(TokenType::Colon, "after attribute name")?;
        let value_token: Token = self.expect(TokenType::Value, "for attribute value")?;

        Ok(Attribute {
            name: name_token.content,
            value: value_token.content,
            line: name_token.line,
            column: name_token.column,
        })
    }

    fn parse_styles(&mut self) -> Result<Vec<Style>, ParserError> {
        self.expect(TokenType::StyleStart, "to start a style block")?;

        let mut styles = Vec::new();
        styles.push(self.parse_style()?);

        while self.is_current_token_type(&TokenType::Comma) {
            self.advance();
            styles.push(self.parse_style()?);
        }

        self.expect(TokenType::StyleEnd, "to close a style block")?;

        Ok(styles)
    }

    fn parse_style(&mut self) -> Result<Style, ParserError> {
        let name_token = self.expect(TokenType::Identifier, "for style property name")?;
        self.expect(TokenType::Colon, "after style property name")?;
        let value_token = self.expect(TokenType::Value, "for style property value")?;

        Ok(Style {
            name: name_token.content,
            value: value_token.content,
            line: name_token.line,
            column: name_token.column,
        })
    }
}