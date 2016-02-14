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

// fn advent5_part2() {
//   let input = read_file("advent5.txt");
//   let mut a = 0;

//   for line in input.split('\n') {
//     if !contains_naughties(line) && contains_nices(line) && contains_vowels(line) {
//       a += 1;
//     }
//   }

// }