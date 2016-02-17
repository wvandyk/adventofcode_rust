use std::collections::HashMap;
use general;

fn deliver_presents(instructions: &Vec<char>, houses: &mut HashMap<(i32, i32), i32>) {
  let mut pos = (0, 0);

  houses.insert(pos, 1);

  for &d in instructions {
    match d {
      '^' => pos.1 += 1,
      'v' => pos.1 -= 1,
      '>' => pos.0 += 1,
      '<' => pos.0 -= 1,
      _   => {},
    }

    match houses.get(&pos) {
      Some(&presents) => { houses.insert(pos, presents + 1); },
      None            => { houses.insert(pos, 1); }
    }
  }

  return ()
}

pub fn advent3_part1() {
  let input = general::read_file("advent3.txt");
  let mut houses: HashMap<(i32, i32), i32> = HashMap::new();

  houses.insert((0, 0), 1);
  deliver_presents(&input.chars().collect(), &mut houses);

  println!("Advent 3, Part 1: Houses that got at least 1 present: {}", houses.len());
}

pub fn advent3_part2() {
  let input = general::read_file("advent3.txt");
  let mut sins: Vec<char> = vec![];
  let mut rins: Vec<char> = vec![];

  let mut houses: HashMap<(i32, i32), i32> = HashMap::new();

  houses.insert((0, 0), 2);

  let mut i = 0;

  for c in input.chars() {
    if i % 2 == 0 {
      sins.push(c);
    } else {
      rins.push(c);
    }

    i += 1;
  }

  deliver_presents(&sins, &mut houses);
  deliver_presents(&rins, &mut houses);

  println!("Advent 3, Part 2: Houses that got at least 1 present: {}", houses.len());
}
