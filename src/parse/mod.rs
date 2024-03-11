use crate::ast::{Statement, AST};

use self::{lexeme::{Lexeme, LexemeKind}, scanner::Scanner};

pub mod lexeme;
pub mod scanner;

pub trait Parse: Sized {
    fn parse(parser: &mut Parser<'_>) -> ParseResult<Self>;
}

pub type ParseResult<T> = anyhow::Result<T>;

pub struct Parser<'a> {
    ast: AST,
    lexemes: std::iter::Peekable<Scanner<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            ast: AST::new(),
            lexemes: Scanner::new(source).peekable(),
        }
    }

    pub fn run(mut self) -> ParseResult<AST> {
        while let Some(peeked) = self.peek_raw_lexeme() {
            match peeked.kind {
                LexemeKind::Whitespace(_) | LexemeKind::Comment(_) => { self.lexemes.next().expect("missing lexeme"); },
                _ => {
                    let stmt = Statement::parse(&mut self)?;
                    self.ast.add_stmt(stmt);
                },
            }
        }
        return Ok(self.ast);
    }

    pub fn next_raw_lexeme(&mut self) -> Option<Lexeme> {
        self.lexemes.next()
    }

    pub fn peek_raw_lexeme(&mut self) -> Option<&Lexeme> {
        self.lexemes.peek()
    }

    pub fn push_label_to_ast(&mut self, label: Box<str>) -> usize {
        self.ast.add_label(label)
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
