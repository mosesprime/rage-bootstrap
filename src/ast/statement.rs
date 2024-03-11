use anyhow::{bail, Context};

use crate::{parse::{lexeme::LexemeKind, Parse, ParseResult, Parser}, token::keyword::KeywordKind, Token};

use super::{declaration::Declaration, expression::Expression};

#[derive(Debug)]
pub enum Statement {
    Declaration(Declaration),
    Expression(Expression),
    Return(ReturnStmt),
}

impl Parse for Statement {
    fn parse(parser: &mut Parser<'_>) -> ParseResult<Self> {
        let lexeme = parser.peek_raw_lexeme().context("statement missing lexeme")?;
        match lexeme.kind {
            LexemeKind::Keyword(KeywordKind::Return) => Ok(Statement::Return(ReturnStmt::parse(parser)?)),
            LexemeKind::Term 
                | LexemeKind::Keyword(KeywordKind::Mut)
                | LexemeKind::Keyword(KeywordKind::Pub) => Ok(Statement::Declaration(Declaration::parse(parser)?)),
            _ => bail!("invalid start to a new statement: {:?}", lexeme),
        }
    }
}
#[derive(Debug)]
pub struct ReturnStmt {
    pub return_tok: Token![return],
    pub inner_expr: Expression,
}

impl Parse for ReturnStmt {
    fn parse(parser: &mut Parser<'_>) -> ParseResult<Self> {
        parser.next_raw_lexeme().expect("missing lexeme");
        let inner_expr = Expression::parse(parser)?;
        Ok(ReturnStmt { return_tok: Token!(return), inner_expr })
    }
}
