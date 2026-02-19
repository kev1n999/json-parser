mod lexer;
mod parser;
mod utils;

fn main() {
  let source = utils::read_json::read_json("src/json/example.json");

  match source {
    Ok(src) => {
      let mut lexer = lexer::Lexer::new(&src);
      let tokens = lexer::Lexer::get_tokens(&mut lexer);
      let mut parser = parser::Parser::parser(tokens);
      let parsed = parser.object_parse();

      if let parser::JsonObject::Object(obj) = parsed {
        if let Some(val) = obj.get("usuario") {
          println!("{}", val);
        }
      }
    },
    Err(err) => panic!("An error ocurred: {}", err),
  }
}
