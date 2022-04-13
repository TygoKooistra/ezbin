use super::*;

#[test]
fn basic() {
  let c = parse(String::from("20u8"));
  assert_eq!(c, vec![20u8]);
}