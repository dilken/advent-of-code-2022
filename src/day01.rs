use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use lib; 

pub fn get_elf_carrying_most_kcal(){
    println!("Welcome to day 1!");
    print_elf_kcals(1);
    print_elf_kcals(3);
}

fn print_elf_kcals(top_num_of_elves: i32) -> std::io::Result<()> {
    let mut contents: String = lib::get_input("inputs/day01_input.txt"); 

    let mut max_sums: Vec<i32> = Vec::new();
    let mut min_index = 0;
    let mut v_len = 0;
    
    for elf in contents.split("\n\n") {
        let mut v: Vec<i32> = Vec::new();

        for cals_for_elf in elf.split("\n") {
            v.push(cals_for_elf.parse().unwrap()); 
        }

        let sum: i32 = v.iter().sum();

        if v_len < top_num_of_elves {
            max_sums.push(sum);
            min_index = lib::arg_min(max_sums.clone());
            v_len += 1;
        }
        else if sum > max_sums[min_index as usize] {
            max_sums[min_index as usize] = sum;
            min_index = lib::arg_min(max_sums.clone());
        }
    }

    let top_sum: i32 = max_sums.iter().sum();
    
    println!("The top {} elf(s) carrying the most is carrying: {} kcals", top_num_of_elves, top_sum);

    Ok(())
}


