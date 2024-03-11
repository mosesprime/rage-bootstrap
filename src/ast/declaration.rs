use crate::{parse::{lexeme::LexemeKind, Parse, ParseResult, Parser}, token::{delimiter::{DelimiterKind, Parenthesis}, keyword::KeywordKind, punctuation::Comma}, Token};

use super::{Enclosed, Label, Seperated};

#[derive(Debug)]
pub enum Declaration {
    Function(FuncDecl),
}

impl Parse for Declaration {
    fn parse(parser: &mut Parser<'_>) -> ParseResult<Self> {
        todo!("wip declaration parsing")
    }
}

#[derive(Debug)]
pub struct FuncDecl {
    pub pub_tok: Option<Token!(pub)>,
    pub label: Label,
    pub fn_tok: Token!(fn),
    // TODO: func generics
    pub args: Enclosed<Parenthesis, Seperated<Token!(,), FuncArgExpr>>
    // TODO: ( ... ) <ret_type> { ... }
}

impl Parse for FuncDecl {
    fn parse(parser: &mut Parser<'_>) -> ParseResult<Self> {
        let mut lexeme = parser.next().expect("missing lexeme");
        let mut pub_tok: Option<Token!(pub)> = None;
        if lexeme.kind == LexemeKind::Keyword(KeywordKind::Pub) {
            pub_tok = Some(Token!(pub));
        }
        let label = Label::parse(parser)?;
        todo!("wip FuncDecl parsing");
    }
}

#[derive(Debug)]
pub struct FuncArgExpr;
