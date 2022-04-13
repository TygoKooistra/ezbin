use std::fs;

fn main() {
  println!("Hello, world!");
}

fn parse_file(path: String) {
  let code = fs::read_to_string(path)
    .expect("Could not load a file");
  
  for c in code.chars() {
    
  }
}