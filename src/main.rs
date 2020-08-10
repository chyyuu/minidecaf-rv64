fn main() -> std::io::Result<()> {
  let path = std::env::args()
    .nth(1)
    .expect("usage: minidecaf <input path>");
  //let path = "examples/step2/not_zero.c".to_string();
  minidecaf::run(path, &mut std::io::stdout())
}
