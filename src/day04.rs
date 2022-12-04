use crate::utils; 
use itertools::Itertools;

pub fn print_elf_sections() { 
  let input = utils::get_input("inputs/day04_input.txt"); 

  let sections = input
      .lines()
      .filter_map(|l| {
        l.split(",")
        .flat_map(|p| p.split("-").filter_map(|x| x.parse::<i32>().ok()))
        .collect_tuple()
        })
      .collect_vec();

  let first_overlap = sections
      .iter()
      .filter(|(a1, a2, b1, b2)| (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2))
      .count();

  let second_overlap = sections
      .iter()
      .filter(|(a1, a2, b1, b2)| (a1 <= b1 && b1 <= a2) || (b1 <= a1 && a1 <= b2))
      .count();

  println!("Elf sections first part: {}", first_overlap.to_string());
  println!("Elf sections second part: {}", second_overlap.to_string());
}
