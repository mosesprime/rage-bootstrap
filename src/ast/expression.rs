use anyhow::bail;

use crate::{parse::{lexeme::LexemeKind, Parse, ParseResult, Parser}, token::LiteralKind};

#[derive(Debug)]
pub enum Expression {
    Literal(LiteralExpr)
}

impl Parse for Expression {
    fn parse(parser: &mut Parser<'_>) -> ParseResult<Self> {
        // TODO: parse expressions
        Ok(Expression::Literal(LiteralExpr::parse(parser)?))
    }
}

#[derive(Debug)]
pub struct LiteralExpr {
    pub kind: LiteralKind,
    pub value: Box<str>,
}

impl Parse for LiteralExpr {
    fn parse(parser: &mut Parser<'_>) -> ParseResult<Self> {
        let lexeme = parser.next().expect("missing lexeme");
        if let LexemeKind::Literal(kind) = lexeme.kind {
            Ok(LiteralExpr { kind, value: lexeme.value.expect("missing value") })
        } else {
            bail!("can not parse as literal: {:?}", lexeme);   
        }
    }
}


