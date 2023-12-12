use advent_23::read_lines;

fn main() {
    let unknown = read_lines("data/day_04.txt")
        .into_iter()
        .map(|card| Card::from(card))
        .map(|card| card.score())
        .reduce(|total, c_score| total + c_score);
}


struct Card {
    index: i32,
    winning_numbers: [i32; 10],
    our_numbers: [i32; 25]
}

impl Card {
    fn from(input: String) -> Card {
        Card { index: 0, winning_numbers: [0;10], our_numbers: [0;25] }
    }

    fn score(&self) -> i32 {
        let Card {index, winning_numbers, our_numbers} = self;
        our_numbers
            .into_iter()
            .filter(|number| Card::is_winner(&winning_numbers, **number))
            .fold(0, |acc, _e| acc * 2)
    }

    fn is_winner(winning_numbers: &[i32;10], input: i32) -> bool {
        match winning_numbers
            .iter()
            .position(|&x| x == input) {
                Some(_) => true,
                None => false
        }
    }
}

#[cfg(test)]
mod tests_4x1 {
    use crate::Card;

    #[test]
    fn card_scoring() {
        vec![
            Card {index: 0, winning_numbers: [1;10], our_numbers: [0;25] },
            Card {index: 0, winning_numbers: [1;10], our_numbers: [0;25] },
            Card {index: 0, winning_numbers: [1;10], our_numbers: [0;25] },
            Card {index: 0, winning_numbers: [1;10], our_numbers: [0;25] },
            Card {index: 0, winning_numbers: [1;10], our_numbers: [0;25] },          
        ]
        .iter()
        .map(|card| card.score())
        .zip(vec![0;5].into_iter())
        .for_each(|(input, test)| {
            assert_eq!(input, test);
        })
    }
}