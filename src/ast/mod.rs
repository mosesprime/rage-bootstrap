use anyhow::{bail, Context};

use crate::{parse::{lexeme::LexemeKind, Parse, ParseResult, Parser}, token::delimiter::DelimiterKind, Token};

pub use self::statement::Statement;

mod declaration;
mod expression;
mod statement;

#[derive(Debug)]
pub struct AST {
    stmts: Vec<Statement>,
    label_cursor: usize,
    labels: Vec<Box<str>>
}

impl AST {
    pub fn new() -> Self {
        Self { 
            stmts: vec![],
            label_cursor: 0,
            labels: vec![],
        }
    }

    pub fn add_stmt(&mut self, stmt: Statement) {
        log::debug!("adding statement: {stmt:?}");
        self.stmts.push(stmt)
    }

    pub fn add_label(&mut self, label: Box<str>) -> usize {
        log::debug!("adding label: {label}");
        self.labels.push(label);
        let i = self.label_cursor;
        self.label_cursor += 1;
        i
    }
}

#[derive(Debug)]
pub struct Label(usize);

impl Parse for Label {
    fn parse(parser: &mut Parser<'_>) -> ParseResult<Self> {
        let lexeme = parser.next().expect("missing lexeme");
        let value = lexeme.value.expect("missing value");
        match lexeme.kind {
            LexemeKind::Term => Ok(Label(parser.push_label_to_ast(value))),
            _ => bail!("unable to parse into label: {}", value),
        }
    }
}

#[derive(Debug)]
pub struct Enclosed<D, T> {
    pub delimiter: D,
    pub inner: T,
}

#[derive(Debug)]
pub struct Seperated<S, T> {
    pub seperator: S,
    pub inner: Vec<T>,
}
