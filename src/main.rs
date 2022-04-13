use std::env;
use std::fs;


fn print_usage() {
  print!("EZbin usage:
<filename>    Load a file into the processing queue
-o <filename> Set output file
");
}

fn main() {
  let mut args: Vec<String> = env::args().collect();
  args.remove(0);




  let mut lastArg = String::from("");
  for arg in args {

  }



}

fn parse_file(path: String) {
  let code = fs::read_to_string(path)
    .expect("Could not load a file");
  
  for c in code.chars() {

  }
}