// use regex::Regex;
// use std::collections::HashMap;
// use general;

// pub fn advent7_part1() {
// }

// fn get_wires(circuit: &str) -> HashMap<&str, u16> {
//   let mut wires: HashMap<&str, u16> = HashMap::new();
//   let re = Regex::new(r"->\s([a-z]+)$").unwrap();
//   for line in circuit.split('\n') {
//     let caps = re.captures(line).unwrap();
//     let wname = caps.at(1).unwrap();
//     wires.insert(wname, 0);
//   }
//   return wires;
// }

// #[test]
// fn test_get_wires() {
//   let circ = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i";
//   assert_eq!(['x', 'y', 'z'], get_wires(circ).keys().collect::<Vec<_> >() );
// }