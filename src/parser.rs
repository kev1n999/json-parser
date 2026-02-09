use crate::lexer;
use std::collections::{HashMap};

#[derive(Debug)]
enum JsonObject {
  Null,
  Bool(bool),
  String(String),
  Number(f64),
  Array(Vec<JsonObject>),
  Object(HashMap<String, JsonObject>)
}

pub struct Parser {
  tokens: Vec<lexer::Token>,
  current: usize,
}

impl Parser {
  pub fn parser(tokens: Vec<lexer::Token>) -> Self {
    Parser { tokens: tokens, current: 0, }
  }

  pub fn parse_object(&mut self) -> JsonObject {
    match self.peek() {
      Some(token) => {
        match token.token_type {
          lexer::TokenKind::String(s) => self.string_parse(),
          lexer::TokenKind::Number(n) => self.number_parse(),
        }
      }
    }
  }

  fn peek(&self) -> Option<&lexer::Token> {
    self.tokens.get(self.current)
  }

  fn advance_current(&mut self) {
    self.current += 1;
  }

  fn string_parse(&mut self) -> JsonObject {
    let peek_token = self.peek();
    if let Some(token) = peek_token {
      if let lexer::TokenKind::String(s) = &token.token_type {
        let value = s.to_string();
        self.advance_current();
        return JsonObject::String(value);
      }
    }
    JsonObject::Null
  }

  fn number_parse(&mut self) -> JsonObject {
    let peek_token = self.peek();
    if let Some(token) = peek_token {
      if let lexer::TokenKind::Number(n) = &token.token_type {
        return JsonObject::Number(n.clone());
      }
    }
    JsonObject::Null
  }
}
