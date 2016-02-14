use regex::Regex;
use std;
use general;

fn turn_on(lights: &mut Vec<bool>, lx: u32, ly: u32, rx: u32, ry: u32) {
  for y in ly..(ry + 1) {
    for x in lx..(rx + 1) {
      let index = x + y * 1000;
      lights[index as usize] = true;
    }
  }
}

fn turn_off(lights: &mut Vec<bool>, lx: u32, ly: u32, rx: u32, ry: u32) {
  for y in ly..(ry + 1) {
    for x in lx..(rx + 1) {
      let index = x + y * 1000;
      lights[index as usize] = false;
    }
  }
}

fn toggle(lights: &mut Vec<bool>, lx: u32, ly: u32, rx: u32, ry: u32) {
  for y in ly..(ry + 1) {
    for x in lx..(rx + 1) {
      let index = x + y * 1000;
      lights[index as usize] = !lights[index as usize];
    }
  }
}

pub fn advent6_part1() {
  let input = general::read_file("advent6.txt");
  let re = Regex::new(r"([^0-9]*)\s(\d+),(\d+)\sthrough\s(\d+),(\d+)").unwrap();
  let mut lights: Vec<bool> = std::iter::repeat(false).take(1000001).collect::<Vec<bool>>();

  for line in input.split('\n') {
    let caps = re.captures(line).unwrap();
    let instruction = caps.at(1).unwrap();
    let lx:u32 = caps.at(2).unwrap().parse().unwrap();
    let ly:u32 = caps.at(3).unwrap().parse().unwrap();
    let rx:u32 = caps.at(4).unwrap().parse().unwrap();
    let ry:u32 = caps.at(5).unwrap().parse().unwrap();

    match instruction {
      "turn on"  => turn_on(&mut lights, lx, ly, rx, ry),
      "turn off" => turn_off(&mut lights, lx, ly, rx, ry),
      "toggle"   => toggle(&mut lights, lx, ly, rx, ry),
      _          => {}
    }
  }

  let mut acc = 0;
  for l in lights {
    match l {
      true => acc += 1,
      false => {}
    }
  }

  println!("Advent 6, Part 1: {}", acc);
}

fn turn_on_p2(lights: &mut Vec<u32>, lx: u32, ly: u32, rx: u32, ry: u32) {
  for y in ly..(ry + 1) {
    for x in lx..(rx + 1) {
      let index = x + y * 1000;
      lights[index as usize] += 1;
    }
  }
}

fn turn_off_p2(lights: &mut Vec<u32>, lx: u32, ly: u32, rx: u32, ry: u32) {
  for y in ly..(ry + 1) {
    for x in lx..(rx + 1) {
      let index = x + y * 1000;
      if lights[index as usize] == 0 {} else { lights[index as usize] -= 1 }
    }
  }
}

fn toggle_p2(lights: &mut Vec<u32>, lx: u32, ly: u32, rx: u32, ry: u32) {
  for y in ly..(ry + 1) {
    for x in lx..(rx + 1) {
      let index = x + y * 1000;
      lights[index as usize] += 2;
    }
  }
}

pub fn advent6_part2() {
  let input = general::read_file("advent6.txt");
  let re = Regex::new(r"([^0-9]*)\s(\d+),(\d+)\sthrough\s(\d+),(\d+)").unwrap();
  let mut lights: Vec<u32> = std::iter::repeat(0).take(1000001).collect::<Vec<u32>>();

  for line in input.split('\n') {
    let caps = re.captures(line).unwrap();
    let instruction = caps.at(1).unwrap();
    let lx:u32 = caps.at(2).unwrap().parse().unwrap();
    let ly:u32 = caps.at(3).unwrap().parse().unwrap();
    let rx:u32 = caps.at(4).unwrap().parse().unwrap();
    let ry:u32 = caps.at(5).unwrap().parse().unwrap();

    match instruction {
      "turn on"  => turn_on_p2(&mut lights, lx, ly, rx, ry),
      "turn off" => turn_off_p2(&mut lights, lx, ly, rx, ry),
      "toggle"   => toggle_p2(&mut lights, lx, ly, rx, ry),
      _          => {}
    }
  }

  let mut acc = 0;
  for l in lights {
    acc += l;
  }

  println!("Advent 6, Part 2: {}", acc);
}