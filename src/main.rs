pub mod ast;
pub mod codegen;
pub mod ir;
pub mod lexer;
pub mod parser;

fn main() -> std::io::Result<()> {
  let path = std::env::args()
    .nth(1)
    .expect("usage: minidecaf <input path>");
  //let path = "examples/step1/return_0.c".to_string();
  minidecaf::run(path, &mut std::io::stdout())
}
