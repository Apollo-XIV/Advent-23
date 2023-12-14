use crate::read_lines;
use std::cmp::min;

pub fn part_1(cards: &[Card;201]) -> usize {
    cards.iter()
        .map(|card| card.score())
        .sum()
}

pub fn part_2(cards: &[Card;201]) -> usize {
    let mut totals: [usize; 201] = [1; 201];
    for i in 0..201 {
        cards[i].copies.iter()
            .for_each(|&index| totals[index] += totals[i]);
    }
    totals.into_iter().sum()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    matches: Vec<usize>,
    copies: Vec<usize>
}

impl Card {
    fn from(input: String) -> Card {
        let parse = |input: &str| input.split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        let [index, wants, haves]: [&str; 3] = input.split(":").flat_map(|string| string.split("|")).collect::<Vec<&str>>().try_into().expect("failed to parse)");
        let index: usize = index[5..]
            .trim().parse::<usize>()
            .expect("found non integer") -1;
        let matches: Vec<usize> = parse(haves).into_iter()
            .filter(|&x| parse(wants).iter().position(|&y| x==y).is_some())
            .collect();

        let start = min(200, index+1);
        let end = min(200, index+matches.len());
        let copies: Vec<usize> = if index != 200 {(start..=end).collect()} else {vec![]};

        Card {matches, copies}
    }

    fn score(&self) -> usize {
        match self.matches.len() {
            0 => 0,
            x => 2_usize.pow(x as u32 -1)
        }
    }
}

pub fn gen_card_lib() -> [Card; 201] {
    read_lines("data/day_04.txt").into_iter()
    .map(|input| Card::from(input)).collect::<Vec<Card>>().try_into().expect("wrong number of items")
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn part_1_test() {
        assert_eq!(part_1(&gen_card_lib()), 20855);
    }
    
    #[test]
    pub fn part_2_test() {
        assert_eq!(part_2(&gen_card_lib()), 5489600);
    }

    fn gen_test_cards() -> [Card; 5] {
        [
            Card {
                matches: vec![35,75],
                copies: vec![19, 20]
            },
            Card {
                matches: vec![5, 81, 37, 14, 49, 77, 29, 72, 74, 3],
                copies: vec![47, 48, 49, 50, 51, 52, 53, 54, 55, 56]
            },
            Card {
                matches: vec![15, 24],
                copies: vec![48, 49]
            },
            Card {
                matches: vec![47],
                copies: vec![129]
            },
            Card {
                matches: vec![52, 26],
                copies: vec![197, 198]
            }          
        ]
    }

    #[test]
    fn card_scoring() {
        gen_test_cards()
            .into_iter()
            .map(|card| card.score())
            .zip(TEST_SCORES.iter().copied())
            .for_each(|(input, test)| assert_eq!(input, test))
    }
    
    #[test]
    fn parsing() {
        TEST_INPUTS
            .iter()
            .map(|&input| Card::from(input.to_string()))
            .zip(gen_test_cards().into_iter())
            .for_each(|(input, test)| assert_eq!(input, test));
    }
    
    const TEST_INPUTS: [&str; 5] = [
        "Card  19: 87 38 27 92 35 94 88 75 37 74 | 89  7 24 54  9 98 13 42 32 60  8  6 90 35 75 18 68 96 80 59 44 85 95 21 17",
        "Card  47: 72 37 74 81 14  3 29 77  5 49 |  5 88 89 81 37 14 71 95 63 54 49 24 67 62 77 29 72 92 39 80 60 74 59  3 93",
        "Card  48: 75 24  3 20 15 42 98 80 71 99 | 72 81 27 78 96 44 37 91 65 30  4 93 64 15 28 34 48 39 38 66 57 45 24 47  5",
        "Card 129: 86 33 98 10 56 71 70 47 31 38 |  4 52 20 65 93 74 92 77 46 58 48  9 59 27 45 69 47 83 88 40 95 73 76 21 35",
        "Card 197: 52 78 18 87 19 20 26 50 37 67 | 16 52 96 77 13 99 48 53 86 98 51 71 84 81 41  5 25 27 29  3 72 26 22 68  2"
    ];
    const TEST_SCORES: [usize; 5] = [2, 512, 2, 1, 2];
}