use crate::read_lines;

fn main() {
    let lines = read_lines("data/day_05.txt");
}

pub fn part_1() {}

pub fn part_2() {}

pub struct Map {
    range: Range<u32>,
    diff: i32,
}
impl Map {
    fn from() -> Map {
        Map {
            from: (0..100),
            to: String::from("test"),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn scratch() {
        println!("{:?}", TEST_INPUT.split("\n"))
    }

    #[test]
    pub fn part_1_test() {
        assert_eq!(part_1(), ());
    }

    #[test]
    pub fn part_2_test() {
        assert_eq!(part_2(), ());
    }

    const TEST_INPUT: str = "seeds: 79 14 55 13

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
