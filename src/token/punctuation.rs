use std::str::FromStr;

use anyhow::anyhow;

macro_rules! define_punctuation {
    ($($token:literal $name:ident)*) => {
        $(
            #[doc = concat!('`', $token, '`')]
            pub struct $name;
        )*

        #[derive(Debug, PartialEq)]
        pub enum PunctuationKind {
            $(
                #[doc = concat!('`', $token, '`')]
                $name,
            )*
        }

        impl FromStr for PunctuationKind {
            type Err = anyhow::Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($token => Ok(PunctuationKind::$name),)*
                    _ => Err(anyhow!("unable to parse {}", s)),
                }
            }
        }
    };
}

define_punctuation!(
    "!" Exclamation
    // "
    "#" Pound
    "$" Dollar
    "%" Percent
    "&" Andpersand
    // '
    "(" LParen
    ")" RParen
    "*" Asterisk
    "+" Plus
    "," Comma
    "-" Hyphen
    "." Dot
    "/" Slash
    ":" Colon
    ";" SemiColon
    "<" Lesser
    "=" Equal
    ">" Greater
    "?" Question
    "@" At 
    "[" LSquare
    "\\" Backslash
    "]" RSquare
    "^" Caret
    "_" Underscore
    // `
    "{" LCurly
    "|" Pipe
    "}" RCurly
    "~" Tilde

    "==" EqualEqual
    "!=" NotEqual
    ">=" GreaterEqual
    "<=" LesserEqual
    "&&" AndAnd
    "||" OrOr
    "<<" LeftShift
    ">>" RightShift
    "+=" PlusEqual
    "-=" MinusEqual
    ".." DotDot
    "..=" DotDotEqual
    "..." DotDotDot
);
