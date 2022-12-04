use crate::utils; 

pub fn print_elf_carrying_most_kcal(){
    print_elf_kcals(1).unwrap();
    print_elf_kcals(3).unwrap();
}

fn print_elf_kcals(top_num_of_elves: i32) -> std::io::Result<()> {
    let contents = utils::get_input_file_contents("inputs/day01_input.txt"); 

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
            min_index = utils::arg_min(&max_sums);
            v_len += 1;
        }
        else if sum > max_sums[min_index as usize] {
            max_sums[min_index as usize] = sum;
            min_index = utils::arg_min(&max_sums);
        }
    }

    let top_sum: i32 = max_sums.iter().sum();
    
    println!("The top {} elf(s) carrying the most is carrying: {} kcals", top_num_of_elves, top_sum);

    Ok(())
}



