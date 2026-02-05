mod lexer;

fn main() {
  let source = r#"{"string_test": "abcd", "test": -4.3 }"#;
  let mut lexer = lexer::Lexer::new(source);

  lexer.display();
}
