
mod generate_ast;

use generate_ast::generate_ast;
fn main() -> std::io::Result<()> {
    generate_ast("src")
}