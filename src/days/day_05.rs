use crate::read_lines;
use itertools::Itertools;
use std::{
    cmp::{max, min},
    ops::Range,
};

pub fn part_1() -> i64 {
    let input = read_lines("data/day_05.txt").join("\n");
    let (seeds, maps) = parse_input(&input);
    seeds
        .iter()
        .map(|&seed| maps.iter().fold(seed, |seed, map| map.convert_int(seed)))
        .min()
        .expect("Couldn't find a minimum value")
    //println!("{:?}", result);
}

pub fn part_2() -> i64 {
    let input = read_lines("data/day_05.txt").join("\n");
    let (seeds, maps) = parse_input(&input);
    let seed_ranges = parse_seeds(seeds);
    maps.iter()
        .fold(seed_ranges, |seed_ranges, map| {
            seed_ranges
                .into_iter()
                .flat_map(|seed_range| map.convert_range(seed_range))
                .collect()
        })
        .iter()
        .map(|range| range.start)
        .min()
        .expect("failed to find min")
    //println!("{:?}", min);
}

#[derive(Debug)]
/// A collection of non-overlapping conversions.
struct Map(Vec<Conv>);

impl FromIterator<Conv> for Map {
    fn from_iter<T: IntoIterator<Item = Conv>>(iter: T) -> Self {
        let mut map = iter.into_iter().collect::<Vec<Conv>>();
        map.sort_by_key(|x| x.range.start);
        map.insert(0, Conv::null(i64::MIN..i64::MIN));
        map.push(Conv::null(i64::MAX..i64::MAX));
        map = intersperse_ranges(map);
        map.remove(0);
        map.pop();
        Map(map)
    }
}

fn intersperse_ranges(map: Vec<Conv>) -> Vec<Conv> {
    let mut result: Vec<Conv> = Vec::new();

    // Iterate through each range
    for conv in map {
        // Fill the gap before the current range
        if let Some(last) = result.last().cloned() {
            if last.range.end < conv.range.start {
                result.push(Conv::null(last.range.end..conv.range.start));
            }
        }

        // Insert the current range
        result.push(conv);
    }

    result
}

impl Map {
    fn convert_int(&self, input: i64) -> i64 {
        match self.0.iter().find(|&conv| conv.range.contains(&input)) {
            Some(x) => x.diff + input,
            None => input,
        }
    }

    fn convert_range(&self, seed_range: Range<i64>) -> Vec<Range<i64>> {
        self.0
            .iter()
            .filter_map(|conv| conv.apply_on_intersection(&seed_range))
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Conv {
    range: Range<i64>,
    diff: i64,
}
impl Conv {
    fn null(range: Range<i64>) -> Conv {
        Conv { range, diff: 0 }
    }

    fn from(input: &str) -> Conv {
        let [dest, source, range]: [i64; 3] = input
            .split_ascii_whitespace()
            .map(|x| x.parse().expect("INVALID INPUT"))
            .collect::<Vec<i64>>()
            .try_into()
            .expect("Invalid Number of Inputs");

        Conv {
            range: (source..source + range),
            diff: dest - source,
        }
    }

    fn apply_on_intersection(&self, range: &Range<i64>) -> Option<Range<i64>> {
        if range.end < self.range.start || range.start > self.range.end {
            return None;
        }
        let start = max(range.start, self.range.start) + self.diff;
        let end = min(range.end, self.range.end) + self.diff;
        Some(start..end)
    }
}

fn parse_input(string: &str) -> (Vec<i64>, Vec<Map>) {
    let mut iter = string.split("\n\n");
    let seeds = iter.next().expect("EMPTY :(")[6..] // start reading from 6 chars in
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let maps = iter
        .map(|x: &str| x.split('\n').skip(1).map(Conv::from).collect())
        .collect();
    (seeds, maps)
}

fn parse_seeds(input: Vec<i64>) -> Vec<Range<i64>> {
    input
        .into_iter()
        .tuples()
        .map(|(k, k2)| k..k + k2)
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn scratch() {
        let input = read_lines("data/day_05.txt").join("\n");
        let (seeds, maps) = parse_input(&input);
        let result = seeds
            .iter()
            .map(|&seed| maps.iter().fold(seed, |seed, map| map.convert_int(seed)))
            .min()
            .expect("Couldn't find a minimum value");

        println!("{:?}", result);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(), 322500873);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(), 108956227);
    }

    #[test]
    fn test_input() {
        let (seeds, maps) = parse_input(TEST_INPUT);
        let result = seeds
            .iter()
            .map(|&seed| maps.iter().fold(seed, |seed, map| map.convert_int(seed)))
            .min()
            .expect("Couldn't find a minimum value");

        println!("{:?}", result);
    }

    #[test]
    fn test_input_part_2() {
        let (seeds, maps) = parse_input(TEST_INPUT);
        let seed_ranges = parse_seeds(seeds);
        println!("seeds: {:?} \nmaps: {:?}", seed_ranges, maps);
        let min = maps
            .iter()
            .fold(seed_ranges, |seed_ranges, map| {
                dbg!(&seed_ranges);
                let result = seed_ranges
                    .into_iter()
                    .flat_map(|seed_range| map.convert_range(seed_range))
                    .collect();
                result
            })
            .iter()
            .map(|x| x.start)
            .min()
            .expect("couldn't find a min");

        println!("{:?}", min);
    }

    #[test]
    fn parsing_inputs() {
        let (seeds, maps) = parse_input(TEST_INPUT);
        println!("{:?}", maps);
        assert_eq!(seeds, [79, 14, 55, 13]);
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
