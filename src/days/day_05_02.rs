use crate::read_lines;
use std::ops::Range;
use itertools::Itertools;

fn part_1() {
    let input = read_lines("data/day_05.txt").join("\n");
    let (seeds, maps) = parse_input(&input);
    let result = seeds.iter()
        .map(|&seed| maps
            .iter()
            .fold(seed, |seed, map| map.convert_int(seed))
        )
        .min().expect("Couldn't find a minimum value");
    println!("{:?}", result);
}

fn part_2() {
    let input = read_lines("data/day_05.txt").join("\n");
    let (seeds, maps) = parse_input(&input);
    let seeds = parse_seeds(seeds);
    let result = seeds.iter()
        .map(|&seed| maps
            .iter()
            .fold(seed, |seed, map| map.convert_int(seed))
        )
        .min().expect("Couldn't find a minimum value");
    println!("{:?}", result);
}

#[derive(Debug)]
/// A collection of non-overlapping conversions.
struct Map(Vec<Conv>);

impl Map {
    fn convert_int(&self, input: i64) -> i64{
        match self.0.iter().find(|&conv| conv.range.contains(&input)) {
            Some(x) => x.diff + input,
            None => input
        }
    }
}

#[derive(Debug)]
struct Conv {
    range: Range<i64>,
    diff: i64
}
impl Conv {
    fn from(input: &str) -> Conv {
        let [dest, source, range]: [i64;3] = input
            .split_ascii_whitespace()
            .map(|x| x.parse().expect("INVALID INPUT"))
            .collect::<Vec<i64>>().try_into().expect("Invalid Number of Inputs");
        
        Conv {
            range: (source..source+range),
            diff: dest-source
        }
    }

    fn apply(&self, input: i64) -> i64 {
        if !&self.range.contains(&input) {return input};
        input + self.diff
    }

}

fn parse_input(string: &str) -> (Vec<i64>, Vec<Map>) {
    let mut iter = string.split("\n\n");
    let seeds = iter.next().expect("EMPTY :(")[6..] // start reading from 6 chars in
        .split_ascii_whitespace()
        .map(|x| x.parse().expect(&format!("invalid character: {}",x)))
        .collect();
    let maps = iter.map(|x: &str| Map(x
            .split('\n')
            .skip(1)
            .map(Conv::from)
            .collect::<Vec<Conv>>())
        )
        .collect::<Vec<Map>>();
    (seeds, maps)
}

fn parse_seeds(input: Vec<i64>) -> Vec<i64> {
    input
        .into_iter()
        .tuples()
        .flat_map(|(k, k2)| (k..k+k2).collect::<Vec<i64>>())
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn scratch() {
        let seeds = parse_seeds(vec![1367444651, 99920667]);
        println!("{:?}",seeds.len());
    }

    #[test]
    fn part_1_test() {
        part_1()
    }

    #[test]
    fn test_input() {
        let (seeds, maps) = parse_input(TEST_INPUT);
        let result = seeds.iter()
            .map(|&seed| maps
                .iter()
                .fold(seed, |seed, map| map.convert_int(seed))
            )
            .min().expect("Couldn't find a minimum value");

        println!("{:?}", result);
    }

    #[test]
    fn test_input_part_2() {
        let (seeds, maps) = parse_input(TEST_INPUT);
        let seeds = parse_seeds(seeds);
        let result = seeds.iter()
            .map(|&seed| maps
                .iter()
                .fold(seed, |seed, map| map.convert_int(seed))
            )
            .min().expect("Couldn't find a minimum value");

        println!("{:?}", result);
    }

    #[test]
    fn parsing_inputs() {
        let (seeds, maps) = parse_input(TEST_INPUT);
        println!("{:?}", maps);
        assert_eq!(seeds,[79, 14, 55, 13]);
    }

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
}
