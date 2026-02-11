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

  fn peek(&self) -> Option<&lexer::Token> {
    self.tokens.get(self.current)
  }

  fn advance_current(&mut self) {
    self.current += 1;
  }

  pub fn object_parse(&mut self) -> JsonObject {
    match self.peek() {
      Some(token) => {
        match &token.token_type {
          lexer::TokenKind::String(s) => self.string_parse(),
          lexer::TokenKind::Number(n) => self.number_parse(),
          lexer::TokenKind::LeftBracket => self.array_parse(),
        }
      },
      None => panic!("an error ocurred"),
    }
  }

  fn array_parse(&mut self) -> JsonObject {
    match self.peek() {  
      Some(token) => {
        if let lexer::TokenKind::LeftBracket = &token.token_type {
          self.advance_current(); 
        } else { return JsonObject::Null; }
      },
      None => panic!("eof!"),
    }

    let mut json_objects: Vec<JsonObject> = Vec::new();

    loop {
      match self.peek() {
        Some(token) => {
          if let lexer::TokenKind::RightBracket = &token.token_type {
            self.advance_current();
            break;
          }

          let parse_object: JsonObject = self.object_parse();
          json_objects.push(parse_object);

          match self.peek() {
            Some(token) => {
              if let lexer::TokenKind::Comma = &token.token_type {
                self.advance_current();
              } else {
                if token.token_type != lexer::TokenKind::RightBracket {
                  panic!("Syntax error!");
                } else {
                  self.advance_current();
                  break; 
                }
              }
            },
            None => panic!("eof!"),
          }
        },
        None => panic!("eof!"),
      }
    }
    JsonObject::Array(json_objects)
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
        let number = *n;
        self.advance_current();
        return JsonObject::Number(number);

      }
    }
    JsonObject::Null
  }
}
