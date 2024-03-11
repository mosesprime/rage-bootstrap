use self::scanner::Scanner;

mod lexeme;
pub mod scanner;

pub trait Parse: Sized {
    fn parse() -> ParseResult<Self>;
}

pub type ParseResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct Parser<'a> {
    scanner: Scanner<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            scanner: Scanner::new(source),
        }
    }

    pub fn run(self) {
        self.scanner.for_each(|l| println!("{l:?}"));
    }
}
