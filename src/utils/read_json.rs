use std::fs::File;
use std::io::prelude::*;

pub fn read_json(file: &str) -> std::io::Result<String> {
  let mut f = File::open(file)?;
  let mut json_content = String::new();
  f.read_to_string(&mut json_content)?;
  Ok(json_content)
}
