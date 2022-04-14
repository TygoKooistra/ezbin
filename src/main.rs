use std::io::prelude::*;
use std::env;
use std::fs;
use std::thread;

use std::process;
use std::str::FromStr;
use std::vec::IntoIter;

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

  let big_endian = true;

  let mut in_string = false;
  let mut string = String::from("");

  let mut in_value_start = true;
  let mut value_start = String::from("");
  let mut value_type = String::from("");

  let mut last_c = '\0';
  for c in code.chars() {
    if in_string {
      if last_c == '\\' {
        match c {
          '"' => { string.push('"'); },
          '\\'=> { string.push('\\'); last_c='\0'; continue; },
          'n' => { string.push('\n'); },
          _ => { eprintln!("Escape character closed with '{}' (not recognized)", c); process::exit(1); }
        }
      }else if c != '\\' {
        if c == '"' {
          value_type.push(c);
          in_string = false;
          in_value_start = false;
        }else {
          string.push(c);
        }
      }
    }else if in_value_start {
      if (c >= '0' && c <= '9') || c == '-' || c == '+' {
        value_start.push(c);
      }else if c == '"' {
        in_string = true;
      }else {
        if c.is_whitespace() {
          continue;
          //in_value_start = false;
        }else {
          match c {
            'i' => { in_value_start = false; }
            'u' => { in_value_start = false; },
            _ => { eprintln!("Unknown type initializer: {}", c); process::exit(2)}
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
          },
          "u16" => {
            bytes.reserve(2);
            let num = u16::from_str(value_start.as_str())
              .expect("Number parsing error");
            let bs = if big_endian { num.to_be_bytes() } else { num.to_be_bytes() };

            bs.into_iter()
              .for_each(|v| { bytes.push(v); });
          }
          "u32" => {
            bytes.reserve(4);
            let num = u32::from_str(value_start.as_str())
              .expect("Number parsing error");
            let bs = if big_endian { num.to_be_bytes() } else { num.to_be_bytes() };

            bs.into_iter()
              .for_each(|v| { bytes.push(v); });
          }
          "u64" => {
            bytes.reserve(8);
            let num = u64::from_str(value_start.as_str())
              .expect("Number parsing error");
            let bs = if big_endian { num.to_be_bytes() } else { num.to_be_bytes() };

            bs.into_iter()
              .for_each(|v| { bytes.push(v); });
          }
          "i8" => {
            bytes.push(
              i8::from_str(value_start.as_str())
                .expect("Number parsing error") as u8
            );
          },
          "i16" => {
            bytes.reserve(2);
            let num = i16::from_str(value_start.as_str())
              .expect("Number parsing error");
            let bs = if big_endian { num.to_be_bytes() } else { num.to_be_bytes() };

            bs.into_iter()
              .for_each(|v| { bytes.push(v); });
          }
          "i32" => {
            bytes.reserve(4);
            let num = i32::from_str(value_start.as_str())
              .expect("Number parsing error");
            let bs = if big_endian { num.to_be_bytes() } else { num.to_be_bytes() };

            bs.into_iter()
              .for_each(|v| { bytes.push(v); });
          }
          "i64" => {
            bytes.reserve(8);
            let num = i64::from_str(value_start.as_str())
              .expect("Number parsing error");
            let bs = if big_endian { num.to_be_bytes() } else { num.to_be_bytes() };

            bs.into_iter()
              .for_each(|v| { bytes.push(v); });
          }

          "\"UTF8" => {
            if value_start.len() != 0 {
              panic!("impropper use of strings");
            }

            let bs = string.bytes();
            bytes.reserve(bs.len());
            for b in bs {
              bytes.push(b);
            }
          }
          "\"ASCII" => {
            if value_start.len() != 0 {
              panic!("impropper use of strings");
            }
            if !string.is_ascii() {
              eprintln!("String is not correct ascii ({})", string);
              process::exit(2);
            }

            let bs = string.bytes();
            bytes.reserve(bs.len());
            for b in bs {
              bytes.push(b);
            }
          }
          "\"UTF16" => {
            if value_start.len() != 0 {
              panic!("impropper use of strings");
            }

            let utf16 = string.encode_utf16();
            for c in utf16 {
              bytes.reserve(2);
              let bs = if big_endian { c.to_be_bytes() } else { c.to_be_bytes() };

              bs.into_iter()
                .for_each(|v| { bytes.push(v); });
            }
          }
          _ => { panic!("Unknown type"); }
        }

        in_value_start = true;
        value_start = String::from("");
        value_type = String::from("");
        string = String::from("");
      }else {
        value_type.push(c);
      }
    }
    last_c = c;
  }

  return bytes;
}