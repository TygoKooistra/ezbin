use super::*;

use std::path::*;

fn test_example(p: &str) -> bool {
  let path = Path::new(p);
  if !path.exists() {
    eprintln!("Example {} does not exist!", path.to_string_lossy());
    return false;
  }
  if !path.is_dir() {
    eprintln!("Example {} is a file (and not a directory)", path.to_string_lossy());
    return false;
  }
  
  let expect = path.join("output.bin");
  let expect_string = expect.clone().to_string_lossy().to_string();
  let out = path.join(".temp.out.bin");
  let out_string = out.clone().to_string_lossy().to_string();
  
  if out.exists() {
    eprintln!("{} was not removed, please remove it manually", out.to_string_lossy());
    return false;
  }


  let mut toload_add = path.file_name()
    .expect("Could not read file name")
    .to_os_string();
  toload_add.push(".ezbin");

  let toload = path.join(Path::new(&toload_add));

  let result = load(Vec::from([
    String::from( "-o" ),
      out.to_string_lossy().to_string(),
      toload.to_string_lossy().to_string(),
  ]));
  if result.is_err() {
    return false;
  }

  if !out.exists() {
    eprintln!("Ouput file was not created");
    return false;
  }

  let res = fs::read(out.clone());
  if res.is_err() {
    eprintln!("Could not read {}", out_string);
    return false;
  }
  let exp = fs::read(expect);
  if res.is_err() {
    eprintln!("Could not read {}", expect_string);
    return false;
  }

  if res.unwrap() != exp.unwrap() {
    eprintln!("Files {} and {} do NOT match!", out_string, expect_string);
    return false;
  }

  let rem = fs::remove_file(out.clone());
  if rem.is_err() {
    eprintln!("COULD NOT REMOVE {}", out.to_string_lossy());
    return true;
  }

  return true;
}

#[test]
fn test_basic() {
  assert_eq!(
    test_example("./examples/basic"),
    true
  );
}

#[test]
fn test_defaults() {
  assert_eq!(
    test_example("./examples/defaults"),
    true
  );
}

#[test]
fn test_endian() {
  assert_eq!(
    test_example("./examples/endian"),
    true
  );
}

