use general;

pub fn advent1_part1() {

  let s = general::read_file("advent1.txt");
  let mut a = 0;

  for item in s.chars() {
    match item {
      '(' => a += 1,
      ')' => a -= 1,
      _   => {}
    }
  }

  println!("Advent 1, Part 1: Final floor: {}", a);
}

pub fn advent1_part2() {
  let s = general::read_file("advent1.txt");
  let mut a = 0;
  let mut i = 1;

  for item in s.chars() {
    match item {
      '(' => a += 1,
      ')' => a -= 1,
      _   => {}
    }

    if a == -1 {
      break;
    }

    i += 1;
  }

  println!("Advent 1, Part 2: Character position that enters basement: {}", i);
}
