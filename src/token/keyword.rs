use std::str::FromStr;

use anyhow::anyhow;

macro_rules! define_keyword {
    ($($token:literal $name:ident)*) => {
        $(
            pub struct $name;

            impl $name {
                pub fn as_kind(&self) -> KeywordKind {
                    KeywordKind::$name
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
                    $(concat!('"', $token, '"') => Ok(KeywordKind::$name),)*
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
