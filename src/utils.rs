use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn get_input(input_file_path: &str) -> String {
    let file = File::open(input_file_path).expect("Could not read input file!");
    let mut buf_reader = BufReader::new(file);
    let mut input_file_contents = String::new();
    buf_reader.read_to_string(&mut input_file_contents).unwrap();

    input_file_contents
}

pub fn arg_min(v: &Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    let mut index: i32 = 0;
    let mut min = &i32::MAX;
    for e in v {
        if e < min {
            min = e;
            result = index;
        }
        index += 1;
    }
    
    result
}
