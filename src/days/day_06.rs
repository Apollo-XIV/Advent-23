use itertools::Itertools;
use roots::find_roots_quadratic;
use roots::Roots;
use std::fs::read_to_string;
use std::ops::RangeInclusive;

pub fn part_1() -> i64 {
    parsing::pt1(&read_to_string("data/day_06.txt").unwrap())
        .iter()
        .map(|race| sizeof_range(race.record_breakers().expect("couldnt find roots")))
        .product()
}

pub fn part_2() -> i64 {
    sizeof_range(
        parsing::pt2(&read_to_string("data/day_06.txt").unwrap())
            .record_breakers()
            .expect("couldn't find roots"),
    )
}

#[derive(PartialEq, Eq, Debug)]
struct Race {
    time: i64,
    distance: i64,
}
impl Race {
    fn record_breakers(&self) -> Option<RangeInclusive<i64>> {
        match find_roots_quadratic(-1f64, self.time as f64, -self.distance as f64 - 0.1) {
            Roots::Two([lower, upper]) => Some((lower.ceil() as i64)..=(upper.floor() as i64)),
            _ => None,
        }
    }
}

fn sizeof_range(range: RangeInclusive<i64>) -> i64 {
    range.end() + 1 - range.start()
}

mod parsing {
    use super::*;

    pub fn pt1(input: &str) -> Vec<Race> {
        let [times, distances] = input
            .split('\n')
            .map(|line| {
                line.split_ascii_whitespace()
                    .skip(1)
                    .map(|value| value.parse().expect("INVALID INPUT"))
                    .collect::<Vec<i64>>()
            })
            .collect::<Vec<Vec<i64>>>()
            .try_into()
            .expect("INVALID INPUT");

        times
            .iter()
            .zip(distances.iter())
            .map(|(&time, &distance)| Race { time, distance })
            .collect()
    }

    pub fn pt2(input: &str) -> Race {
        let [time, distance] = input
            .split('\n')
            .map(|line| {
                line.split_ascii_whitespace()
                    .skip(1)
                    .join("")
                    .parse()
                    .expect("invalid input")
            })
            .collect::<Vec<i64>>()
            .try_into()
            .expect("wrong no. of lines");
        Race { time, distance }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn part_2_test() {
        assert_eq!(23501589, part_2())
    }

    #[test]
    fn part_1_test() {
        assert_eq!(1083852, part_1());
    }

    #[test]
    fn test_possibilities() {
        let count: i64 = races()
            .iter()
            .map(|race| sizeof_range(race.record_breakers().expect("couldnt find roots")) as i64)
            .product();
        assert_eq!(count, 288)
    }

    #[test]
    fn find_record_breakers() {
        assert_eq!(
            2..=5,
            Race {
                time: 7,
                distance: 9
            }
            .record_breakers()
            .expect("couldnt find roots")
        );
        assert_eq!(
            4..=11,
            Race {
                time: 15,
                distance: 40
            }
            .record_breakers()
            .expect("couldnt find roots")
        );
        assert_eq!(
            11..=19,
            Race {
                time: 30,
                distance: 200
            }
            .record_breakers()
            .expect("couldnt find roots")
        );
    }

    #[test]
    fn parsing() {
        assert_eq!(races(), parsing::pt1(TEST_INPUT))
    }

    #[test]
    fn sizeof_range_tests() {
        assert_eq!(sizeof_range(2..=5), 4);
        assert_eq!(sizeof_range(4..=11), 8);
        assert_eq!(sizeof_range(11..=19), 9);
    }

    #[test]
    fn parsing_pt2() {
        assert_eq!(
            Race {
                time: 71530,
                distance: 940200
            },
            parsing::pt2(TEST_INPUT)
        )
    }

    fn races() -> Vec<Race> {
        vec![
            Race {
                time: 7,
                distance: 9,
            },
            Race {
                time: 15,
                distance: 40,
            },
            Race {
                time: 30,
                distance: 200,
            },
        ]
    }

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";
}
