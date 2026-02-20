# Json Parser
It is a simple json parser written in rust

---

# How to use
1. Clone the repository
```
git clone https://github.com/kev1n999/json-parser
```
2. Open the project
```
cd json-parser
```
3. Run
```
cargo run
```
---

# More
1. You can write specific a json in `json/example.json` to parser
2. You can select a specific json object in `main.rs`

## For example
```rust
if let parser::JsonObject::Object(obj) = parsed {
  // Select the object by "user" key in the json source/file
  if let Some(val) = obj.get("user") {
     println!("{}", val); // to display the value of object
    }
  }
```

---
