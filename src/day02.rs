use crate::utils; 

pub fn print_rock_paper_scissors_scores() {
    print_scores().unwrap(); 
}

fn print_scores() -> std::io::Result<()> {
    let input = utils::get_input("inputs/day02_input.txt"); 
    let part_one: u32 = input
            .lines()
            .into_iter()
            .fold(0u32, |score, line| match line {
                "A X" => score + 4,
                "A Y" => score + 8,
                "A Z" => score + 3,
                "B X" => score + 1,
                "B Y" => score + 5,
                "B Z" => score + 9,
                "C X" => score + 7,
                "C Y" => score + 2,
                "C Z" => score + 6,
                _ => unreachable!(),
            })
            .into();

    println!("Your total score for part_one is: {}", part_one);


    let part_two: u32 = input
            .lines()
            .into_iter()
            .fold(0u32, |score, line| match line {
                "A X" => score + 3,
                "B X" => score + 1,
                "C X" => score + 2,
                "A Y" => score + 4,
                "B Y" => score + 5,
                "C Y" => score + 6,
                "A Z" => score + 8,
                "B Z" => score + 9,
                "C Z" => score + 7,
                _ => unreachable!(),
            })
            .into();

    println!("Your total score for part_two is: {}", part_two);

    Ok(())
}