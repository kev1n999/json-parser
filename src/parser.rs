use crate::lexer;
use std::collections::{HashMap};
use std::fmt; 

#[derive(Debug)]
pub enum JsonObject {
  Null,
  Bool(bool),
  String(String),
  Number(f64),
  Array(Vec<JsonObject>),
  Object(HashMap<String, JsonObject>)
}

#[derive(Debug)]
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
      Some(token) => match &token.token_type {
        lexer::TokenKind::String(_) => self.string_parse(),
        lexer::TokenKind::Number(_) => self.number_parse(),
        lexer::TokenKind::LeftBracket => self.array_parse(),
        lexer::TokenKind::RightBracket => {
          self.advance_current();
          self.object_parse()
        }
        lexer::TokenKind::True => {
          self.advance_current();
          JsonObject::Bool(true)
        }
        lexer::TokenKind::False => {
          self.advance_current();
          JsonObject::Bool(false)
        }
        lexer::TokenKind::LeftBrace => self.json_object_parse(),
        lexer::TokenKind::RightBrace => {
          self.advance_current();
          JsonObject::Null
        }
        lexer::TokenKind::Colon => {
          self.advance_current();
          self.object_parse()
        }
        lexer::TokenKind::Comma => {
          self.advance_current();
          self.object_parse()
        }
        lexer::TokenKind::Null => {
          self.advance_current();
          JsonObject::Null
        }
        lexer::TokenKind::EOF => {
          self.json_object_parse()
        }
      },
      None => panic!("EOF inesperado"),
    }
  }

  fn json_object_parse(&mut self) -> JsonObject {
    match self.peek() {
      Some(token) => {
        if !matches!(&token.token_type, lexer::TokenKind::LeftBrace) {
          return JsonObject::Null; 
        }
        self.advance_current();
      },
      None => panic!("eof!"),
    }

    let mut object_hasher: HashMap<String, JsonObject> = HashMap::new();

    loop {
      match self.peek() {
        Some(token) => {
          if let lexer::TokenKind::RightBrace = &token.token_type {
            self.advance_current();
            break;
          }

          let mut object_key = String::new();
          if let lexer::TokenKind::String(key) = &token.token_type {
            object_key.push_str(key);
            self.advance_current();
          } else { panic!("expected string to key"); }

          match self.peek() {
            Some(token) => {
              if let lexer::TokenKind::Colon = &token.token_type {
                self.advance_current();
              } else {
                panic!("expected ':'!");
              }
            },
            None => panic!("eof!"),
          }

          let object_val = self.object_parse();
          object_hasher.insert(object_key, object_val);

          if let Some(token) = self.peek() {
            if let lexer::TokenKind::Comma = &token.token_type {
              self.advance_current();
            } else if matches!(&token.token_type, lexer::TokenKind::RightBrace) { self.advance_current(); break; }
          }
        },
        None => panic!("eof!"),
      }
    }
    JsonObject::Object(object_hasher)
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
