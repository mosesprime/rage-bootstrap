use std::str::FromStr;

use anyhow::{anyhow, bail};

use crate::parse::{lexeme::LexemeKind, Parse, ParseResult, Parser};

macro_rules! define_keyword {
    ($($token:literal $name:ident)*) => {
        $(
            #[derive(Debug)]
            pub struct $name;

            impl $name {
                pub fn as_kind(&self) -> KeywordKind {
                    KeywordKind::$name
                }
            }

            impl Parse for $name {
                fn parse(parser: &mut Parser<'_>) -> ParseResult<Self> {
                    let lexeme = parser.next().expect("missing lexeme");
                    if lexeme.kind != LexemeKind::Keyword(KeywordKind::$name) {
                        bail!("unable to parse keyword: {lexeme:?}");
                    }            
                    Ok($name)
                }
            }
        )*

        #[derive(Debug, PartialEq)]
        pub enum KeywordKind {
            $(
                #[doc = concat!('`', $token, '`')]
                $name,
            )*
        }

        impl FromStr for KeywordKind {
            type Err = anyhow::Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($token => Ok(KeywordKind::$name),)*
                    _ => Err(anyhow!("unable to parse {}", s)),
                }
            }
        }
    };
}

define_keyword!(
    "alias" Alias
    "break" Break
    "continue" Continue
    "else" Else
    "enum" Enum
    "fn" Fn
    "for" For
    "if" If
    "impl" Impl
    "loop" Loop
    "match" Match
    "mod" Mod
    "mut" Mut
    "pub" Pub
    "return" Return
    "Self" SelfType
    "self" SelfValue
    "struct" Struct
    "trait" Trait
    "type" Type
    "union" Union
    "use" Use
    "while" While
);

