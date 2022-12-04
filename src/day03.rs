use crate::utils; 

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

fn prio(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        (c as u32) - ('a' as u32) + 1
    } else {
        (c as u32) - ('A' as u32) + 27
    }
}

fn get_compartment_items(backpack_items: &str) -> (&str, &str) {
    let half_char_len = backpack_items.len()/2;
    let first = &backpack_items[..half_char_len];
    let second = &backpack_items[backpack_items.len()-half_char_len..];

    (first, second)
}
 
fn print_backpack_badges() {
    let contents = utils::get_input_file_contents("inputs/day03_input.txt"); 
    let mut v: Vec<String> = Vec::new();

    for x in contents.lines() {
        v.push(x.parse().unwrap()); 
    }

    let backpacks: Vec<Vec<u8>> = v.iter().map(|s| s.as_bytes().to_vec()).collect();
    let mut result = 0;

    for i in (0..backpacks.len()).step_by(3) {
        let (s1, s2, s3) = (&backpacks[i], &backpacks[i + 1], &backpacks[i + 2]);
        for c in s1 {
            if s2.contains(c) && s3.contains(c) {
                result += get_backpack_prio(c);
                break;
            }
        }
    }
    println!("Badges in elf backpacks: {}", result);
}

fn get_backpack_prio(c: &u8) -> u32 {
    match c {
        b'a'..=b'z' => (c - b'a' + 1) as u32,
        b'A'..=b'Z' => (c - b'A' + 27) as u32,
        _ => 0,
    }
}