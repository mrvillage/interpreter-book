mod ast;
mod eval;
mod lexer;
mod object;
mod parser;
mod repl;

pub use ast::*;
pub use lexer::*;
pub use parser::*;
pub use repl::*;

fn main() {
    start_repl(std::io::stdin(), std::io::stdout());
}
