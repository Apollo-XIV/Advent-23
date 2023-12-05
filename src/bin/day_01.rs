use std::env;
use std::fs;

fn main() {
    // Check the calibration values
    // filter the list by digits
    // if length < 1, duplicate single value
    // create new digit pair from first and last
    // 

    let lines: i32 = read_lines("data/day_01")
        .into_iter()
        .map(|line| filter_numbers(line))
        .map(|line| format!("{}{}", line[0], line[line.len() - 1] ).parse::<i32> ().unwrap())
        .sum::<i32>();

    println!("{:?}",lines);

}


fn filter_numbers(unfiltered: String) -> Vec<i32> {
    let mut output: Vec<i32> = vec![];
    for char in unfiltered.to_owned().split("") {
        match char.parse() {
            Ok(num) => {
                output.push(num);
            },
            Err(_) => {
                continue;
            } 
        }
    }
    output
}

use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}