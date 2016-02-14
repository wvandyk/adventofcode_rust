use crypto::digest::Digest;
use crypto::md5::Md5;

fn tryhash(key: &str, n:u64, c:usize) -> bool {
  let mut md5 = Md5::new();
  md5.input_str(key);
  let ns = format!("{}", n);
  md5.input_str(&ns);

  let hash = md5.result_str();
  hash.bytes().take(c).all(|b| b == b'0')
}

pub fn advent4_part1() {
  let input = "ckczppom";
  let mut i:u64 = 117944;

  while !tryhash(input, i, 5) {
    i += 1;
  }

  println!("Advent 4, Part 1: {}", i);
}

pub fn advent4_part2() {
  let input = "ckczppom";
  let mut i:u64 = 3938036;

  while !tryhash(input, i, 6) {
    i += 1;
  }

  println!("Advent 4, Part 2: {}", i);
}