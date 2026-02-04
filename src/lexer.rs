// Enum to set all tokens to lexer
#[derive(Debug)]
pub enum TokenKind {
  LeftBracket, // [
  RightBracket, // ]
  LeftBrace, // {
  RightBrace, // }
  Comma,
  Colon,

  Null,
  True,
  False,
  String(String),
  Number(f64),

  EOF,
}

#[derive(Debug)]
struct Token {
  token_type: TokenKind,
  lexeme: String,
}

#[derive(Debug)]
pub struct Lexer {
  input: Vec<char>,
  current: usize,
}

impl Lexer {
  pub fn new(source: &str) -> Self {
    Lexer { input: source.chars().collect(), current: 0, }
  }

  fn get_current(&self) -> Option<char> {
    self.input.get(self.current)
  }

  fn advance_current(&mut self) {
    self.current += 1;
  }
}
