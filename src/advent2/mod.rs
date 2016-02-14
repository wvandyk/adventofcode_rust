use general;

fn area(l: i32, w: i32, h: i32) -> i32 {
  let mut nums = vec![l, w, h];
  nums.sort();
  let numlen = nums.len();
  nums.truncate(numlen - 1);
  let slack = nums[0] * nums[1];
  return (2 * l * w) + (2 * w * h) + (2 * h * l) + slack;
}

fn ribbon(l: i32, w: i32, h: i32) -> i32 {
  let mut nums = vec![l, w, h];
  nums.sort();

  let length = (nums[0] * 2) + (nums[1] * 2);
  let bow = nums[0] * nums[1] * nums[2];
  return length + bow;
}

fn advent2_vec_vec() -> Vec< Vec<i32> > {
  let input = general::read_file("advent2.txt");
  let mut nums: Vec < Vec <i32> > = vec![];

  for line in input.split('\n') {
    let linenums: Vec<i32> = 
      line
        .trim()
        .split('x')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();
    nums.push(linenums);
  }

  return nums;
}

pub fn advent2_part1() {
  let nums = advent2_vec_vec();

  let total_area = 
    nums
      .iter()
      .map(|n| area(n[0], n[1], n[2]))
      .fold(0, |x, y| x + y);
  println!("Advent 2, Part 1: Total Area: {}", total_area);
}

pub fn advent2_part2() {
  let nums = advent2_vec_vec();

  let total_ribbon = 
    nums
      .iter()
      .map(|n| ribbon(n[0], n[1], n[2]))
      .fold(0, |x, y| x + y);
  println!("Advent 2, Part 2: Total Ribbon: {}", total_ribbon);
}
