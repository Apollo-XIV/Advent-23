use advent_23::read_lines;
use std::cmp::min;

fn main() {
    let card_lib: [Card; 201] = read_lines("data/day_04.txt")
        .into_iter()
        .map(|card| Card::from(card)).collect::<Vec<Card>>().try_into().expect("wrong number of elems");
    let mut my_cards: Vec<usize> = (1..=201).rev().collect();

    let mut totals: [usize; 201] = [0; 201];
    while let Some(c_indx) = my_cards.pop() {
        card_lib[c_indx-1]
            .get_copies().into_iter()
            .for_each(|x: usize| my_cards.push(x));
        totals[c_indx-1] += 1;
        // println!("processing: {} \t| {} left...", card_lib[c_indx-1].index, my_cards.len())
    }
    println!("{:?}", totals.into_iter().sum::<usize>())
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Card {
    index: usize,
    winning_numbers: [i32; 10],
    our_numbers: [i32; 25]
}

impl Card {


    fn from(input: String) -> Card {
        let stream: Vec<&str> = input.split(":").flat_map(|string| string.split("|")).collect();
        let index: usize = stream
            .get(0).expect("stream was empty")[5..]
            .trim().parse()
            .expect("found non integer");
        let winning_numbers: Vec<i32> = Card::parse_number_string(
            stream.get(1)
                .expect("vec was empty"));
            
        let our_numbers: Vec<i32> = Card::parse_number_string(
            stream.get(2)
                .expect("vec was empty"));

        Card {
            index: index,
            winning_numbers: winning_numbers.try_into().expect("uhoh"),
            our_numbers: our_numbers.try_into().expect("uhoh")
        }
    }


    fn get_copies(&self) -> Vec<usize> {
        let additional = self.our_numbers.into_iter()
            .filter(|&x| self.is_winner(x))
            .collect::<Vec<i32>>().len();
        if additional == 0 || self.index == 201 {
            // println!("caller: {} \t| output: None :(", self.index);
            return vec![]
        }
        // println!("caller: {} \t| output: {:?}", self.index, (self.index+1..=(self.index+additional)));
        let start: usize = min(201, self.index+1);
        let end = min(201, self.index+additional);
        (start..=end).collect() // needs to return 201 max on top range
    }

    fn score(&self) -> i32 {
        self.our_numbers.iter()
            .fold(0, |acc, e| {
                match self.is_winner(*e) {
                    true if acc != 0 => acc * 2,
                    true if acc == 0 => acc + 1,
                    _ => acc
                }
            })   
    }

    fn is_winner(&self, input: i32) -> bool {
        match self.winning_numbers.iter()
            .position(|&x| x == input) {
                Some(_) => true,
                None => false
        }
    }

    fn score_len(&self) -> usize {
        self.our_numbers.iter().copied()
            .filter(|x| self.is_winner(*x))
            .collect::<Vec<i32>>().len()
    }

    fn parse_number_string(input: &str) -> Vec<i32> {
        input
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests_4x2 {
    #[test]
    fn scratch() {
        println!("{:?}",TEST_CARDS[4].score_len());

    }

    #[test]
    fn getting_card_copies() {
        let test_copies: [Vec<usize>;5] = [
            vec![20,21],
            vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57],
            vec![49, 50],
            vec![130],
            vec![198,199]        
        ];

        TEST_CARDS
            .iter()
            .map(|card| card.get_copies())
            .zip(test_copies.iter())
            .for_each(|(input, test)| assert_eq!(input, *test));
    }
    
    #[test]
    fn parsing() {
        TEST_INPUTS
            .iter().copied()
            .map(|input| Card::from(input.to_string()))
            .zip(TEST_CARDS.iter().copied())
            .for_each(|(input, test)| assert_eq!(input, test));
    }

    #[test]
    fn card_scoring() {
        TEST_CARDS
            .iter().copied()
            .map(|card| card.score())
            .zip(TEST_SCORES.iter().copied())
            .for_each(|(input, test)| assert_eq!(input, test))
    }

    use crate::Card;

    

    const TEST_INPUTS: [&str; 5] = [
        "Card  19: 87 38 27 92 35 94 88 75 37 74 | 89  7 24 54  9 98 13 42 32 60  8  6 90 35 75 18 68 96 80 59 44 85 95 21 17",
        "Card  47: 72 37 74 81 14  3 29 77  5 49 |  5 88 89 81 37 14 71 95 63 54 49 24 67 62 77 29 72 92 39 80 60 74 59  3 93",
        "Card  48: 75 24  3 20 15 42 98 80 71 99 | 72 81 27 78 96 44 37 91 65 30  4 93 64 15 28 34 48 39 38 66 57 45 24 47  5",
        "Card 129: 86 33 98 10 56 71 70 47 31 38 |  4 52 20 65 93 74 92 77 46 58 48  9 59 27 45 69 47 83 88 40 95 73 76 21 35",
        "Card 197: 52 78 18 87 19 20 26 50 37 67 | 16 52 96 77 13 99 48 53 86 98 51 71 84 81 41  5 25 27 29  3 72 26 22 68  2"
    ];

    const TEST_CARDS: [Card; 5] = [
        Card {
            index: 19,
            winning_numbers: [87, 38, 27, 92, 35, 94, 88, 75, 37, 74],
            our_numbers: [89, 7, 24, 54, 9, 98, 13, 42, 32, 60, 8, 6, 90, 35, 75, 18, 68, 96, 80, 59, 44, 85, 95, 21, 17]
        },
        Card {
            index: 47,
            winning_numbers: [72, 37, 74, 81, 14, 3, 29, 77, 5, 49],
            our_numbers: [5, 88, 89, 81, 37, 14, 71, 95, 63, 54, 49, 24, 67, 62, 77, 29, 72, 92, 39, 80, 60, 74, 59, 3, 93]
        },
        Card {
            index: 48,
            winning_numbers: [75, 24, 3, 20, 15, 42, 98, 80, 71, 99],
            our_numbers: [72, 81, 27, 78, 96, 44, 37, 91, 65, 30, 4, 93, 64, 15, 28, 34, 48, 39, 38, 66, 57, 45, 24, 47, 5]
        },
        Card {
            index: 129,
            winning_numbers: [86, 33, 98, 10, 56, 71, 70, 47, 31, 38],
            our_numbers: [4, 52, 20, 65, 93, 74, 92, 77, 46, 58, 48, 9, 59, 27, 45, 69, 47, 83, 88, 40, 95, 73, 76, 21, 35]
        },
        Card {
            index: 197,
            winning_numbers: [52, 78, 18, 87, 19, 20, 26, 50, 37, 67],
            our_numbers: [16, 52, 96, 77, 13, 99, 48, 53, 86, 98, 51, 71, 84, 81, 41, 5, 25, 27, 29, 3, 72, 26, 22, 68, 2]
        }          
    ];

    const TEST_SCORES: [i32; 5] = [2, 512, 2, 1, 2];
    

}