use crate::token::{keyword::KeywordKind, punctuation::PunctuationKind, CommentKind, LiteralKind, WhitespaceKind};

#[derive(Debug)]
pub struct Lexeme {
    pub kind: LexemeKind,
    pub length: u32,
    pub index: u32,
    pub value: Option<Box<str>>,
}

impl Lexeme {
    pub fn new(kind: LexemeKind, length: usize, index: usize, value: Option<&str>) -> Self {
        Self {
            kind,
            length: length as u32, 
            index: index as u32,
            value: value.map(|s| s.into()),
        }
    }
}

#[derive(Debug)]
pub enum LexemeKind {
    Whitespace(WhitespaceKind),
    Comment(CommentKind),
    Literal(LiteralKind),
    Punctuation(PunctuationKind),
    Keyword(KeywordKind),
    Term,

    UNKNOWN,
}
