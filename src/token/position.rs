/// Location in source file.
#[derive(Debug)]
pub struct Position {
    /// 1-indexed line number.
    pub line: u32,
    /// 1-indexed character offset.
    pub column: u32,
}

impl Position {
    pub fn new(line: u32, column: u32) -> Self {
        debug_assert!(line > 0);
        debug_assert!(column > 0);
        Self { line, column }
    }
}
