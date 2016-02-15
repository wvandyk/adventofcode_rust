use regex::Regex;
use general;

fn contains_naughties(s: &str) -> bool {
  let naughties = vec!["ab", "cd", "pq", "xy"];

  naughties.iter().any(|x| s.contains(x))
}

fn contains_nices(s: &str) -> bool {
  let nices = (b'a'..b'z' + 1)  // chars as u8 bytes
    .map(|n| format!("{}{}", n as char, n as char)) // doubled and formatted into strings
    .collect::<Vec<String> >(); // in a vector
  
  nices.iter().any(|x| s.contains(x)) 
}

fn contains_vowels(s: &str) -> bool {
  let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
  let mut ss = s.chars().collect::<Vec<char>>();
  ss.retain(|x| vowels.contains(x));
  ss.len() >= 3
}

pub fn advent5_part1() {
  let input = general::read_file("advent5.txt");
  let mut a = 0;

  for line in input.split('\n') {
    if !contains_naughties(line) && contains_nices(line) && contains_vowels(line) {
      a += 1;
    }
  }
  
  println!("Advent 5, Part 1: {}", a);
}

pub fn advent5_part2() {
  let input = general::read_file("advent5.txt");
  let mut a = 0;

  for line in input.split('\n') {
    if repeating_pairs(line) && repeat_with_middle(line) {
      a += 1;
    }
  }

  println!("Advent 5, Part 2: {}", a);
}

fn offset_slices(s: &str, n: usize) -> Vec<&str> {
    (0 .. s.len() - n + 1).map(|i| &s[i .. i + n]).collect()
}

fn repeating_pairs(s: &str) -> bool {
  let ss = offset_slices(s, 2);
  for sub in ss {
    let c = s.matches(sub).count();
    if c > 1 { return true; }
  }

  return false;
}

fn repeat_with_middle(s: &str) -> bool {
  let ss = offset_slices(s, 3);
  let re = Regex::new(r"(.)(.)(.)").unwrap();

  for sub in ss {
    let caps = re.captures(sub).unwrap();

    let c1 = caps.at(1).unwrap();
    let c3 = caps.at(3).unwrap();

    if c1 == c3 {
      return true;
    }

  }
  
  return false;
}

#[test]
fn test_repeating_pairs() {
  assert_eq!(repeating_pairs("aaa"), false);
  assert_eq!(repeating_pairs("xyxy"), true);
  assert_eq!(repeating_pairs("qjhvhtzxzqqjkmpb"), true);
}

#[test]
fn test_repeat_with_middle() {
  assert_eq!(repeat_with_middle("xyx"), true);
  assert_eq!(repeat_with_middle("abcdefeghi"), true);
  assert_eq!(repeat_with_middle("aaa"), true);
  assert_eq!(repeat_with_middle("uurcxstgmygtbstg"), false);
}