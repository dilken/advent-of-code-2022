use crate::utils; 
use itertools::Itertools;

pub fn print_elf_backpack_items() {
    print_backpack_items(); 
    print_backpack_badges(); 
}

fn print_backpack_items() { 
    let contents = utils::get_input_file_contents("inputs/day03_input.txt"); 
    let items: u32 = contents
        .lines()
        .map(get_compartment_items)
        .map(|(s1, s2)| {
            s1.chars().find(|c1| s2.contains(&c1.to_string())).unwrap()
        })
        .map(prio)
        .sum();

    println!("Items in elf backpacks: {}", items);
}

fn print_backpack_badges() {
    let contents = utils::get_input_file_contents("inputs/day03_input.txt"); 

    let badges_count: u32 = contents
        .lines()
        .tuples()
        .map(|(s1, s2, s3)| {
            s1.chars().find(|c1| s2.chars().contains(c1) && s3.chars().contains(c1)).unwrap()
        })
        .map(prio)
        .sum(); 

    println!("Badges in elf backpacks: {}", badges_count); 
}

fn get_compartment_items(backpack_items: &str) -> (&str, &str) {
    let half_char_len = backpack_items.len()/2;
    let first = &backpack_items[..half_char_len];
    let second = &backpack_items[backpack_items.len()-half_char_len..];

    (first, second)
}
 
fn prio(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        (c as u32) - ('a' as u32) + 1
    } else {
        (c as u32) - ('A' as u32) + 27
    }
}