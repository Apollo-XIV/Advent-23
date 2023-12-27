use crate::read_lines;
use itertools::Itertools;
use std::ops::Range;

fn parse_input(string: &str) -> (Vec<i64>, Vec<ConversionBlock>) {
    let mut iter = string.split("\n\n");
    let seeds = iter.next().expect("EMPTY :(")[6..]
        .split_ascii_whitespace()
        .map(|x| x.parse().expect(x))
        .collect();
    let maps = iter.map(|x: &str| ConversionBlock(x
            .split('\n')
            .skip(1)
            .map(Conversion::from)
            .collect::<Vec<Conversion>>())
        )
        .collect::<Vec<ConversionBlock>>();
    (seeds, maps)
}

pub fn part_1() -> i64 {
    let input = read_lines("data/day_05.txt").join("\n");
    let (seeds, map_blocks) = parse_input(&input);
    seeds
    .iter()
    .map(|&seed| map_blocks
        .iter()
        .fold(seed, |acc, e| e.convert(acc)))
        .min().expect("couldnt find minimum value")
}
    
pub fn part_2() -> i64{
    let input = read_lines("data/day_05.txt").join("\n");
    let (seeds, map_blocks) = parse_input(&input);
    let seeds: SeedBlock = parse_seeds(seeds);
    println!("{:?}", seeds);
    1
}

#[derive(Debug)]
pub struct ConversionBlock(Vec<Conversion>);
impl ConversionBlock {
    fn convert(&self, input: i64) -> i64 {
        match self.0.iter()
            .find(|&x| x.range.contains(&input)){
            Some(x) => x.diff + input,
            None => input
        }
    }
    
    // fn that converts a given set of seed ranges through a conversion block
    // the conversion block handles running each seed range through each conversion
    fn convert_set(&self, input: &mut SeedBlock) {
        let mut outputs: Vec<SeedBlock> = vec![];
        // for each layer of the conversion block (each conversion)
        for conversion in self.0.iter() {
            outputs.push(conversion.convert(input));
        };

    }
}

#[derive(Debug)]
pub struct Conversion {
    range: Range<i64>,
    diff: i64,
}

impl Conversion {
    /// Converts a reference to a string into a Conversion struct
    fn from(input: &str) -> Conversion {
        let mut parts = input
            .split_ascii_whitespace()
            .map(|x| x.parse().expect("INVALID INPUT"))
            .collect::<Vec<i64>>();
        let source_range = parts.pop().expect("INVALID NUMBER OF INPUTS");
        let source_start = parts.pop().expect("INVALID NUMBER OF INPUTS");
        let dest_start = parts.pop().expect("INVALID NUMBER OF INPUTS");
        
        Conversion {
            range: (source_start..source_start+source_range),
            diff: dest_start-source_start
        }
    }

    /// Converts any applicable ranges in the seedblock, sperating them out into the return type.
    fn convert(&self, seeds: &mut SeedBlock) -> SeedBlock {
        let mut output: Vec<Range<i64>> = vec![];
        for seed in seeds.0.iter_mut() {
            match seed.extract(&self.range) {
                Some(mut x) => {x.map_diff(self.diff); output.push(x)},
                None => continue,
            }
        };
        SeedBlock(output)
    }
}

#[derive(Clone, Debug)]
pub struct SeedBlock(Vec<Range<i64>>);
impl SeedBlock {
    fn from(input: Range<i64>) -> SeedBlock {
        SeedBlock(vec![input])
    }
}

fn parse_seeds(input: Vec<i64>) -> SeedBlock {
    SeedBlock(input
        .into_iter()
        .tuples()
        .map(|(k, k2)| k..k+k2)
        .collect())
}


trait SeedRange {
    /// Alters a range to shift it by a given amount.
    fn map_diff(&mut self, diff: i64);
    /// Returns a boolean for whether a range overlaps with another.
    fn overlaps(&self, range: &Range<i64>) -> bool;
    /// Takes a range and returns the overlapping range, removing them from the original one
    fn extract(&mut self, range: &Range<i64>) -> Option<Range<i64>>;
}

impl SeedRange for Range<i64> {
    fn map_diff(&mut self, diff: i64) {
        self.start += diff;
        self.end += diff;
    }

    fn overlaps(&self, range: &Range<i64>) -> bool {
        !((self.end < range.start) || (self.start > range.end))
    }

    fn extract(&mut self, range: &Range<i64>) -> Option<Range<i64>> {
        if !self.overlaps(range) {return None}; // early return for no overlap
        return Some(0..1)
    }
}

/// Takes a vec and converts it to find the maximum and minimum values
fn vec_to_range(input: Vec<i64>) -> Option<Range<i64>> {
    if let (Some(min), Some(max)) = (input.iter().min(), input.iter().max()) {
        Some(min.to_owned()..max.to_owned()+1)
    } else {
        None
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn overlapping_ranges() {
        let ranges = vec![
            0..15,
            1..2,
            2..3
        ];
        assert_eq!((0..14).overlaps(&(7..23)), true);
        assert_eq!((70..98).overlaps(&(7..23)), false);
    }

    // #[test]
    // fn converting_ranges() {
    //     let conversion = Conversion {
    //         range: 50..82,
    //         diff: 2
    //     };
    //     println!("{:?}", conversion.convert_range(79..93));
    // }
    
    #[test]
    fn scratch() {
        let (seeds, map_blocks) = parse_input(&TEST_INPUT);
        let mut seeds = parse_seeds(vec![79,14]);
        
    }

    #[test]
    fn example_input() {
        let (seeds, map_blocks) = parse_input(TEST_INPUT);
        seeds
            .iter()
            .map(|&seed| map_blocks
                .iter()
                .fold(seed, |acc, e| e.convert(acc)))
            .zip(TEST_OUTPUTS.iter())
            .for_each(|(input, &test)| assert_eq!(input, test));
    }

    #[test]
    pub fn part_1_test() {
        assert_eq!(part_1(), 322500873);
    }

    #[test]
    pub fn part_2_test() {
        println!("{:?}", part_2())
        // assert_eq!(part_2(), ());
    }

    const TEST_OUTPUTS: [i64; 4] = [82, 43, 86, 35];

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
56 93 4

";
}

