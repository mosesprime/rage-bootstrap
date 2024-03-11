pub use self::statement::Statement;

mod declaration;
mod expression;
mod statement;

#[derive(Debug)]
pub struct AST {
    stmts: Vec<Statement>
}

impl AST {
    pub fn new() -> Self {
        Self { stmts: vec![] }
    }

    pub fn add_stmt(&mut self, stmt: Statement) {
        log::debug!("adding statement: {stmt:?}");
        self.stmts.push(stmt)
    }
}
