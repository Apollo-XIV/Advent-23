use crate::read_lines;

pub fn part_1() -> i32 {
    parse_input("data/day_09.txt")
        .into_iter()
        .map(|sequence| find_next(sequence))
        .sum()
}

pub fn part_2() -> i32 {
    parse_input("data/day_09.txt")
        .into_iter()
        .map(|sequence| find_prev(sequence))
        .sum()
}

fn find_next(list: Vec<i32>) -> i32 {
    // test if list is constant
    if list.iter().all(|&x| x == 0) {
        return 0;
    }
    list.last().unwrap() + find_next(list.diffs())
}

fn find_prev(list: Vec<i32>) -> i32 {
    // test if list is constant
    if list.iter().all(|&x| x == 0) {
        return 0;
    }
    list.first().unwrap() - find_prev(list.diffs())
}

trait Subsequence {
    fn diffs(&self) -> Vec<i32>;
}

impl Subsequence for Vec<i32> {
    fn diffs(&self) -> Vec<i32> {
        self.windows(2)
            .map(|window| window[1] - window[0])
            .collect()
    }
}

fn parse_input(filename: &str) -> Vec<Vec<i32>> {
    read_lines(filename)
        .into_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn part_1_test() {
        assert_eq!(part_1(), 1921197370);
    }

    #[test]
    pub fn part_2_test() {
        assert_eq!(part_2(), 1124);
    }

    #[test]
    fn scratch() {
        let mut sequence: Vec<i32> = vec![
            -6, -7, -7, -6, -4, -1, 3, 8, 14, 21, 29, 38, 48, 59, 71, 84, 98,
            113, //129, 146, 164,
        ];
        sequence.reverse();
        let sequence_2 = vec![1, 4, 9, 16, 25];

        fn find_depth(list: Vec<i32>) -> i32 {
            let next_lvl: Vec<i32> = list.diffs();
            if next_lvl.iter().all(|&x| x == 0) {
                return 0;
            }
            find_depth(next_lvl) + 1
        }

        fn find_prev(list: Vec<i32>) -> i32 {
            // test if list is constant
            if list.iter().all(|&x| x == 0) {
                return 0;
            }
            return dbg!(list.first().unwrap() - find_prev(list.diffs()));
        }

        //dbg!(find_depth(sequence));

        dbg!(find_next(sequence));
    }
}

// /**
//  *
//  *   1   4    9    16     25       (LAST + 11)=36
//  *     3    5    7     9       (LAST + 2)=11
//  *       2    2     2     (LAST + 0)=2
//  *         0    0      0
//  *
//  */
