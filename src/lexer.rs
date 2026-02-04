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
pub struct Token {
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

  pub fn display(&mut self) {
    if self.input.is_empty() {
      panic!("The input is empty!");
    }

    while let Some(characeter) = self.get_current() {
      println!("{:?}", self.next_token());
    }
  }

  fn get_current(&self) -> Option<char> {
    self.input.get(self.current).copied()
  }

  fn advance_current(&mut self) {
    self.current += 1;
  }

  fn ignore_whitespace(&mut self) {
    while let Some(character) = self.get_current() {
      if character.is_whitespace() { self.advance_current(); }
      else { break; }
    }
  }

  fn string_tokenize(&mut self) -> Token {
    let mut string_val = String::new();
    if let Some(character) = self.get_current() {
      if character == '"' {
        self.advance_current();
        while let Some(c) = self.get_current() {
          if c == '"' { break; };
          string_val.push(c);
          self.advance_current();
        }
      }
    }

    Token { token_type: TokenKind::String(string_val.clone()), lexeme: string_val.clone() }
  }

  pub fn next_token(&mut self) -> Token {
    self.ignore_whitespace();

    match self.get_current() {
      Some('{') => {
        self.advance_current();
        Token { token_type: TokenKind::LeftBracket, lexeme: '{'.to_string(), }
      },
      Some('}') => {
        self.advance_current();
        Token { token_type: TokenKind::RightBracket, lexeme: '}'.to_string(), }
      },
      Some('[') => {
        self.advance_current();
        Token { token_type: TokenKind::LeftBrace, lexeme: '['.to_string(), }
      },
      Some(']') => {
        self.advance_current();
        Token { token_type: TokenKind::RightBrace, lexeme: ']'.to_string(), }
      },
      Some(',') => {
        self.advance_current();
        Token { token_type: TokenKind::Comma, lexeme: ','.to_string(), }
      },
      Some(':') => {
        self.advance_current();
        Token { token_type: TokenKind::Colon, lexeme: ':'.to_string(), }
      },
      Some('"') => self.string_tokenize(),

      None => {
        Token { token_type: TokenKind::EOF, lexeme: "".to_string(), }
      }
      _ => panic!("An error ocurred!"),
    }
  }
}
