use crate::ast::{Statement, AST};

use self::{lexeme::{Lexeme, LexemeKind}, scanner::Scanner};

pub mod lexeme;
pub mod scanner;

pub trait Parse: Sized {
    fn parse(parser: &mut Parser<'_>) -> ParseResult<Self>;
}

pub type ParseResult<T> = anyhow::Result<T>;

pub struct Parser<'a> {
    lexemes: std::iter::Peekable<Scanner<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            lexemes: Scanner::new(source).peekable(),
        }
    }

    pub fn run(mut self) -> ParseResult<AST> {
        let mut ast = AST::new();
        while let Some(peeked) = self.peek_raw_lexeme() {
            match peeked.kind {
                LexemeKind::Whitespace(_) | LexemeKind::Comment(_) => { self.lexemes.next().expect("missing lexeme"); },
                _ => ast.add_stmt(Statement::parse(&mut self)?),
            }
        }
        return Ok(ast);
    }

    pub fn next_raw_lexeme(&mut self) -> Option<Lexeme> {
        self.lexemes.next()
    }

    pub fn peek_raw_lexeme(&mut self) -> Option<&Lexeme> {
        self.lexemes.peek()
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next_raw_lexeme()?;
        match next.kind {
            LexemeKind::Whitespace(_) | LexemeKind::Comment(_) => self.next(),
            _ => Some(next),
        }
    }
}
