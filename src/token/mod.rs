pub mod delimiter;
pub mod keyword;
pub mod position;
pub mod punctuation;

#[derive(Debug, PartialEq)]
pub enum WhitespaceKind {
    Blank,
    NewLine,
}

#[derive(Debug, PartialEq)]
pub enum CommentKind {
    /// `//`
    Inline,
    /// `//!`
    Module,
    /// `///`
    Documentation,
}

#[derive(Debug, PartialEq)]
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
    [alias] => { $crate::token::keyword::Alias };
    [break] => { $crate::token::keyword::Break };
    [continue] => { $crate::token::keyword::Continue };
    [else] => { $crate::token::keyword::Else };
    [enum] => { $crate::token::keyword::Enum };
    [fn] => { $crate::token::keyword::Fn };
    [for] => { $crate::token::keyword::For };
    [if] => { $crate::token::keyword::If };
    [impl] => { $crate::token::keyword::Impl };
    [loop] => { $crate::token::keyword::Loop };
    [match] => { $crate::token::keyword::Match };
    [mod] => { $crate::token::keyword::Mod };
    [mut] => { $crate::token::keyword::Mut };
    [pub] => { $crate::token::keyword::Pub };
    [return] => { $crate::token::keyword::Return };
    [Self] => { $crate::token::keyword::SelfType };
    [self] => { $crate::token::keyword::SelfValue };
    [struct] => { $crate::token::keyword::Struct };
    [trait] => { $crate::token::keyword::Trait }; 
    [type] => { $crate::token::keyword::Type };
    [union] => { $crate::token::keyword::Union };
    [use] => { $crate::token::keyword::Use };
    [while] => { $crate::token::keyword::While };

    [!] => { $crate::token::punctuation::Exclamation };
    [#] => { $crate::token::punctuation::Pound };
    [$] => { $crate::token::punctuation::Dollar };
    [%] => { $crate::token::punctuation::Percent };
    [&] => { $crate::token::punctuation::Andpersand };
    [*] => { $crate::token::punctuation::Asterisk };
    [,] => { $crate::token::punctuation::Comma };
    [-] => { $crate::token::punctuation::Hyphen };
    [.] => { $crate::token::punctuation::Dot };
    [/] => { $crate::token::punctuation::Slash };
    [:] => { $crate::token::punctuation::Colon };
    [;] => { $crate::token::punctuation::SemiColon };
    [<] => { $crate::token::punctuation::Lesser };
    [=] => { $crate::token::punctuation::Equal };
    [>] => { $crate::token::punctuation::Greater };
    [?] => { $crate::token::punctuation::Question };
    [@] => { $crate::token::punctuation::At };
    [^] => { $crate::token::punctuation::Caret };
    [_] => { $crate::token::punctuation::Underscore };
    [|] => { $crate::token::punctuation::Pipe };
    [~] => { $crate::token::punctuation::Tilde };
    [==] => { $crate::token::punctuation::EqualEqual };
    [!=] => { $crate::token::punctuation::NotEqual };
    [>=] => { $crate::token::punctuation::GreaterEqual };
    [<=] => { $crate::token::punctuation::LesserEqual };
    [&&] => { $crate::token::punctuation::AndAnd };
    [||] => { $crate::token::punctuation::OrOr };
    [<<] => { $crate::token::punctuation::LeftShift };
    [>>] => { $crate::token::punctuation::RightShift };
    [+=] => { $crate::token::punctuation::PlusEqual };
    [-=] => { $crate::token::punctuation::MinusEqual };
    [..] => { $crate::token::punctuation::DotDot };
    [...] => { $crate::token::punctuation::DotDotDot }; 
    [..=] => { $crate::token::punctuation::DotDotEqual };

    ["("] => { $crate::token::delimiter::Parenthesis };
    [")"] => { $crate::token::delimiter::Parenthesis };
    ["["] => { $crate::token::delimiter::SquareBracket };
    ["]"] => { $crate::token::delimiter::SquareBracket };
    ["{"] => { $crate::token::delimiter::CurlyBrace };
    ["}"] => { $crate::token::delimiter::CurlyBrace };
    ["<"] => { $crate::token::delimiter::AngledBracket };
    [">"] => { $crate::token::delimiter::AngledBracket };
}
