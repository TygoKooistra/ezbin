use std::io::prelude::*;
use std::env;
use std::fs;
use std::thread;

use std::str::FromStr;


#[cfg(test)]
mod tests;


fn print_usage() {
  print!("EZbin usage:
<filename>    Load a file into the processing queue
-o <filename> Set output file
");
}

fn main() {
  let mut args: Vec<String> = env::args().collect();
  args.remove(0);

  if args.len() == 0 {
    print_usage();
    return;
  }


  let mut to_load: Vec<String> = Vec::new();
  let mut output_file: String = String::from("");

  let mut last_arg = String::from("");
  for arg in args {
    if arg == "--help" {
      print_usage();
      return;
    }else if last_arg == "-o" {
      output_file = arg;
      
      last_arg=String::from(""); continue;
    }else if last_arg.len() == 0 {
      if arg != "-o" {
        to_load.push(arg);
        
        last_arg=String::from(""); continue;
      }
    }
    last_arg = arg;
  }

  if output_file.len() == 0 {
    panic!("Please specify an output file with -o");
  }
  
  let mut threads: Vec<thread::JoinHandle<Vec<u8>>> = Vec::new();

  for file in to_load {
    threads.push(
      thread::spawn(|| {
        let code = fs::read_to_string(file)
          .expect("Could not load a file");
        return parse(code);
      })
    );
  }


  let mut written = 0;

  let mut file = fs::File::create(output_file).expect("Could not open the output file");
  for t in threads {
    let bytes = t.join().expect("Internal error while parsing file");
    written = written + bytes.len();

    file.write_all(bytes.as_slice()).expect("Could not write to the output file");
  }

  println!("Wrote {} bytes", written);
}

fn parse(mut code: String) -> Vec<u8> {
  code.push(' ');
  let mut bytes: Vec<u8> = Vec::new();

  let mut in_value_start = true;
  let mut value_start = String::from("");
  let mut value_type = String::from("");

  for c in code.chars() {
    if in_value_start {
      if c >= '0' && c <= '9' {
        value_start.push(c);
      }else {
        if c.is_whitespace() {
          in_value_start = false;
        }else {
          match c {
            'u' => { in_value_start = false; },
            _ => { panic!("Unknown type initializer"); }
          }
          value_type.push(c)
        }
      }
    }else {
      if c.is_whitespace() {
        match value_type.as_str() {
          "u8" => {
            bytes.push(
              u8::from_str(value_start.as_str())
                .expect("Number parsing error")
            );
          }
          _ => { panic!("Unknown type"); }
        }

        in_value_start = true;
        value_start = String::from("");
        value_type = String::from("");
      }else {
        value_type.push(c);
      }
    }
  }

  return bytes;
}