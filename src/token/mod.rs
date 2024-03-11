use self::punctuation::PunctuationKind;

pub mod keyword;
pub mod position;
pub mod punctuation;

#[derive(Debug)]
pub enum TokenKind {
    Literal(LiteralKind),
    Punctuation(PunctuationKind),
    Identifier(),
}

#[derive(Debug)]
pub enum WhitespaceKind {
    Blank,
    NewLine,
}

#[derive(Debug)]
pub enum CommentKind {
    /// `//`
    Inline,
    /// `//!`
    Module,
    /// `///`
    Documentation,
}

#[derive(Debug)]
pub enum LiteralKind {
    Hex,
    Bool,
    Char,
    Binary,
    Float,
    Integer,
    String,
}

#[macro_export]
macro_rules! Token {
    [alias] => { keyword::Alias };
    [break] => { keyword::Break };
    [continue] => { keyword::Continue };
    [&] => { punctuation::Andpersand };
}
