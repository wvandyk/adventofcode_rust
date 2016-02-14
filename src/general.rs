use std::io::prelude::*;
use std::fs::File;

pub fn read_file(filename: &str) -> String {
  let mut f = File::open(filename).unwrap();
  let mut s = String::new();
  f.read_to_string(&mut s).unwrap();
  return s
}