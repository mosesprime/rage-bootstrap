use crate::parse::{Parse, ParseResult, Parser};

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
pub struct FuncDecl {}
