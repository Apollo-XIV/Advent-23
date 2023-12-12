use std::env;
use std::fs;

fn main() {
    // Check the calibration values
    // filter the list by digits
    // if length < 1, duplicate single value
    // create new digit pair from first and last
    // 

    let lines: i32 = read_lines("data/day_01.txt")
        .into_iter()
        .map(|line| filter_numbers(line))
        .map(|line| format!("{}{}", line[0], line[line.len() - 1] ).parse::<i32> ().unwrap())
        .sum::<i32>();

    println!("{:?}",lines);

}

fn wordsearch(target: &mut String) {
    
    let mut bytes = target.clone().into_bytes();
    let number_strings = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    
    for char_index in 0..(bytes.len()-1) { // for every character in the byte string
        for (converted_int, string) in number_strings.iter().enumerate() { // check every number string 
            match bytes.get(char_index..char_index+string.len()) { // index the byte vector
                Some(sample) if sample == string.as_bytes() => bytes[char_index] = (converted_int+1).to_string().into_bytes()[0],                           
                Some(_) => continue,
                None => continue
            }
        }
    }



    println!("{:?}", String::from_utf8_lossy(&bytes).to_string()); 

    *target = String::from_utf8_lossy(&bytes).to_string();
}


fn filter_numbers(unfiltered: String) -> Vec<i32> {
    let mut output: Vec<i32> = vec![];
    let mut unfiltered = unfiltered.to_owned();
    wordsearch(&mut unfiltered);
    for char in unfiltered.to_owned().split("") {
        match char.parse() {
            Ok(num) => output.push(num),
            Err(_) => continue 
        }
    };
    println!("{:?}", output);
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

