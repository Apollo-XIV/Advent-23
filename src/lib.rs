use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


#[cfg(test)]
mod calendar {
    use days;

    #[test]
    fn day_01() {
        use crate::days::day_1x1::*;
        day_1x1::run();
        assert_eq!(4, 4);
    }


    #[test]
    fn day_04() {
        part_1_test();
    }
}
