mod lexer;

fn main() {
  let source = r#"{"name": "kevin"}"#;
  let mut lexer = lexer::Lexer::new(source);

  lexer.display();
}
