use std::str::{Chars, FromStr};

use crate::token::{keyword::KeywordKind, punctuation::PunctuationKind, CommentKind, LiteralKind, WhitespaceKind};

use super::lexeme::{Lexeme, LexemeKind};

/// Lexical tokenizer.
pub struct Scanner<'a> {
    input: &'a str,
    chars: std::iter::Peekable<Chars<'a>>,
    index: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { 
            input: source,
            chars: source.chars().peekable(),
            index: 0,
        }
    }

    fn consume(&mut self, mut predicate: impl FnMut(&char) -> bool) -> usize {
        let mut len = 0;
        while self.chars.next_if(&mut predicate).is_some() {
            len += 1;
        }
        self.index += len;
        return len;
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some(c) = self.chars.next() {
            self.index += 1;
            return Some(c);
        }
        None
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn value(&self, start: usize, end: usize) -> Option<&str> {
        self.input.get(start..end)
    }

    fn index(&self) -> usize {
        self.index
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        let peeked = self.chars.peek()?;
        return match peeked {
            c if c.is_ascii_whitespace() => Some(handle_whitespace(self)),
            c if c.is_ascii_alphabetic() => Some(handle_alphabetic(self)),
            c if c.is_ascii_digit() => Some(handle_numeric(self)),
            c if c.is_ascii_punctuation() => Some(handle_punctuation(self)),
            _ => Some(handle_unknown(self)),
        }
    }
}

fn handle_whitespace(scanner: &mut Scanner<'_>) -> Lexeme {
    let index = scanner.index();
    match scanner.next_char().expect("missing char") {
        '\n' => {
            let length = scanner.consume(|c| c == &'\n');
            Lexeme::new(LexemeKind::Whitespace(WhitespaceKind::NewLine), length, index, None)
        },
        _ => {
            let length = scanner.consume(|c| c.is_whitespace() && c != &'\n');
            Lexeme::new(LexemeKind::Whitespace(WhitespaceKind::Blank), length, index, None)
        },
    }
}

fn handle_alphabetic(scanner: &mut Scanner<'_>) -> Lexeme {
    let index = scanner.index();
    let length = scanner.consume(|c| c.is_ascii_alphanumeric() || c ==  &'_');
    let value = scanner.value(index, index + length).expect("missing value");
    match value {
        "true" | "false" => Lexeme::new(LexemeKind::Literal(LiteralKind::Bool), length, index, Some(value)),
        _ => match KeywordKind::from_str(value) {
            Ok(k) => Lexeme::new(LexemeKind::Keyword(k), length, index, None),
            Err(_) => Lexeme::new(LexemeKind::Term, length, index, Some(value)),
        },
    }
}

fn handle_numeric(scanner: &mut Scanner<'_>) -> Lexeme {
    let index = scanner.index();
    let mut length = scanner.consume(|c| c.is_ascii_digit() || c == &'_');
    match scanner.peek_char() {
        Some('x') => todo!("hex"),
        Some('b') => todo!("binary"),
        Some('.') => todo!("float"),
        _ => {
            let value = scanner.value(index, index + length).expect("missing value");
            Lexeme::new(LexemeKind::Literal(LiteralKind::Integer), length, index, Some(value))
        },
    }
}

fn handle_punctuation(scanner: &mut Scanner<'_>) -> Lexeme {
    let index = scanner.index();
    let mut length = scanner.consume(|c| c.is_ascii_punctuation());
    let mut value = scanner.value(index, index + length).expect("missing value");
    match value {
        "\'" => {
            length += scanner.consume(|c| c != &'\'');
            value = scanner.value(index, index + length).expect("missing value");
            Lexeme::new(LexemeKind::Literal(LiteralKind::Char), length, index, Some(value))
        },
        "\"" => {
            length += scanner.consume(|c| c != &'\"');
            value = scanner.value(index, index + length).expect("missing value");
            Lexeme::new(LexemeKind::Literal(LiteralKind::String), length, index, Some(value))
        },
        "//" => {
            length += scanner.consume(|c| c != &'\n');
            value  = scanner.value(index, index + length).expect("missing value");
            Lexeme::new(LexemeKind::Comment(CommentKind::Inline), length, index, Some(value))
        },
        "//!" => {
            length += scanner.consume(|c| c != &'\n');
            value  = scanner.value(index, index + length).expect("missing value");
            Lexeme::new(LexemeKind::Comment(CommentKind::Module), length, index, Some(value))
        },
        "///" => {
            length += scanner.consume(|c| c != &'\n');
            value  = scanner.value(index, index + length).expect("missing value");
            Lexeme::new(LexemeKind::Comment(CommentKind::Documentation), length, index, Some(value))
        },
       _ => match PunctuationKind::from_str(value) {
            Ok(k) => Lexeme::new(LexemeKind::Punctuation(k), length, index, None),
            Err(_) => Lexeme::new(LexemeKind::UNKNOWN, length, index, Some(value)),
        },
    }
    
}

fn handle_unknown(scanner: &mut Scanner<'_>) -> Lexeme {
    let index = scanner.index();
    scanner.next_char();
    let value = scanner.value(index, index + 1);
    Lexeme::new(LexemeKind::UNKNOWN, 1, index, value)
}
