// Enum to set all tokens to lexer
#[derive(Debug, PartialEq)]
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
  pub token_type: TokenKind,
  pub lexeme: String,
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

  pub fn get_tokens(self: &mut Self) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(_) = self.get_current() {
      tokens.push(self.next_token());
    }

    tokens
  }
  pub fn display(&mut self) {
    if self.input.is_empty() {
      panic!("The input is empty!");
    }

    while let Some(_) = self.get_current() {
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
    while let Some(c) = self.get_current() {
      if c.is_whitespace() {
        self.advance_current();
      } else { break; }
    };
  }

  fn string_tokenize(&mut self) -> Token {
    let mut string_val = String::new();

    if let Some(character) = self.get_current() {
      if character == '"' {
        self.advance_current();
        while let Some(c) = self.get_current() {
          if c == '"' { self.advance_current(); break; };
          string_val.push(c);
          self.advance_current();
        }
      }
    }

    Token { token_type: TokenKind::String(string_val.clone()), lexeme: string_val.clone() }
  }

  fn bool_or_null_tokenize(&mut self) -> Token {
    let mut token = String::new();

    while let Some(character) = self.get_current() {
      if character.is_alphanumeric() {
        token.push(character);
        self.advance_current();
      } else { break; }
    }

    match token.as_str() {
      "true" => Token { token_type: TokenKind::True, lexeme: token, },
      "false" => Token { token_type: TokenKind::False, lexeme: token, },
      "null" => Token { token_type: TokenKind::Null, lexeme: token, },
      _ => panic!("Invalid token: {}", token),
    }
  }


  fn number_tokenize(&mut self) -> Token {
    let mut number_val = String::new();

    if let Some('-') = self.get_current() {
      number_val.push('-');
      self.advance_current();
    }

    let mut has_number = false;

    while let Some(character) = self.get_current() {
      if character.is_ascii_digit() {
        number_val.push(character);
        self.advance_current();
        has_number = true;
      } else { break; }
    }

    if let Some('.') = self.get_current() {
      number_val.push('.');
      self.advance_current();

      let mut has_frac_number = false;
      while let Some(c) = self.get_current() {
        if c.is_ascii_digit() {
          number_val.push(c);
          self.advance_current();
          has_frac_number = true;
        } else { break; }
      }

      if !has_frac_number { panic!("invalid number! {}", number_val); }
    }

    if !has_number { panic!("invalid digit!"); }
    let val = number_val.parse::<f64>().expect("an error ocurred!");
    Token { token_type: TokenKind::Number(val), lexeme: number_val, }
  }

  pub fn next_token(&mut self) -> Token {
    self.ignore_whitespace();

    match self.get_current() {
      Some('{') => {
        self.advance_current();
        Token { token_type: TokenKind::LeftBrace, lexeme: '{'.to_string(), }
      },
      Some('}') => {
        self.advance_current();
        Token { token_type: TokenKind::RightBrace, lexeme: '}'.to_string(), }
      },
      Some('[') => {
        self.advance_current();
        Token { token_type: TokenKind::LeftBracket, lexeme: '['.to_string(), }
      },
      Some(']') => {
        self.advance_current();
        Token { token_type: TokenKind::RightBracket, lexeme: ']'.to_string(), }
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
      Some(c) if c.is_ascii_digit() || c == '-' => self.number_tokenize(),
      Some('t'|'f'|'n') => self.bool_or_null_tokenize(),
      None => {
        Token { token_type: TokenKind::EOF, lexeme: "".to_string(), }
      }
      _ => panic!("An error ocurred!"),
    }
  }
}
