use std::{cmp::Ordering, fs::read_to_string};

pub fn part_1() -> i32 {
    let mut hands = parse_input(&read_to_string("data/day_07.txt").unwrap());
    hands.sort();

    hands
        .iter()
        .zip(1..=hands.len())
        .map(|(hand, rank)| hand.bid * rank as i32)
        .sum::<i32>()
}

pub fn part_2() -> i32 {
    let mut hands = parse_input(&read_to_string("data/day_07.txt").unwrap())
        .iter_mut()
        .map(|hand| hand.upgrade())
        .collect::<Vec<Hand>>();
    hands.sort();
    hands
        .iter()
        .zip(1..=hands.len())
        .map(|(hand, rank)| hand.bid * rank as i32)
        .sum()
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Copy, Clone)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Hand {
    bid: i32,
    cards: [i8; 5], // cards are abstracted into integers
    hand_type: Type,
}

impl Hand {
    fn from(input: &str) -> Hand {
        let [cards, bid]: [String; 2] = input
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .try_into()
            .expect("");

        let bid = bid.parse().expect("invalid bid");

        let cards = cards
            .chars()
            .map(|x| match x {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                x @ '2'..='9' => x.to_digit(10).unwrap() as i8,
                _ => panic!(),
            })
            .collect::<Vec<i8>>()
            .try_into()
            .expect("wrong no. of inputs");

        Hand {
            bid,
            cards,
            hand_type: cards.hand_type(),
        }
    }

    fn upgrade(&mut self) -> Self {
        self.hand_type = (2..=14)
            .map(|insert| self.cards.wildcard(insert).hand_type())
            .max()
            .unwrap();
        self.cards = self.cards.wildcard(1);
        *self
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type.cmp(&other.hand_type).then_with(|| {
            match self
                .cards
                .into_iter()
                .zip(other.cards.into_iter())
                .find(|(a, b)| a != b)
            {
                // find first occurance where a isnt equal to b
                Some((a, b)) if a > b => Ordering::Greater,
                None => Ordering::Equal,
                _ => Ordering::Less,
            }
        })
    }
}

trait PokerHand {
    fn is_x_kind(&self, count: i8) -> bool;
    fn is_two_pair(&self) -> bool;
    fn is_full_house(&self) -> bool;
    fn hand_type(&self) -> Type;
    fn count_of(&self, x: i8) -> i8;
    fn wildcard(&self, insert: i8) -> [i8; 5];
}
impl PokerHand for [i8; 5] {
    fn hand_type(&self) -> Type {
        use Type::*;
        match self {
            hand if hand.is_x_kind(5) => FiveKind,
            hand if hand.is_x_kind(4) => FourKind,
            hand if hand.is_full_house() => FullHouse,
            hand if hand.is_x_kind(3) => ThreeKind,
            hand if hand.is_two_pair() => TwoPair,
            hand if hand.is_x_kind(2) => OnePair,
            _ => HighCard,
        }
    }

    fn is_two_pair(&self) -> bool {
        self.into_iter()
            .copied()
            .filter(|&x| self.count_of(x) == 2)
            .count()
            / 2
            == 2
    }

    fn is_full_house(&self) -> bool {
        let (group_a, group_b): (Vec<i8>, Vec<i8>) = self
            .into_iter()
            .copied()
            .partition(|&x| self.count_of(x) == 3);
        let groups_are_right_length = group_a.len() == 3;
        let group_b_is_homogenous = group_b.iter().all(|&x| x == group_b[0]);
        groups_are_right_length && group_b_is_homogenous
    }

    fn is_x_kind(&self, count: i8) -> bool {
        self.iter().any(|&x| self.count_of(x) == count)
    }

    fn count_of(&self, x: i8) -> i8 {
        self.iter().copied().filter(|&card| card == x).count() as i8
    }

    /// replaces all wildcard values with the insert
    fn wildcard(&self, insert: i8) -> [i8; 5] {
        self.into_iter()
            .copied()
            .map(|value| match value {
                11 => insert,
                x => x,
            })
            .collect::<Vec<i8>>()
            .try_into()
            .unwrap()
    }
}

fn parse_input(input: &str) -> Vec<Hand> {
    input.split('\n').map(Hand::from).collect::<Vec<Hand>>()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(), 243101568);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(), 241344943);
    }

    #[test]
    fn count_of_tests() {
        assert_eq!([1, 2, 2, 4, 5].count_of(2), 2);
        assert_eq!([1, 1, 1, 1, 1].count_of(1), 5);
        assert_eq!([1, 2, 2, 2, 5].count_of(2), 3);
        assert_eq!([1, 2, 3, 3, 3].count_of(3), 3);
    }

    #[test]
    fn test_input_pt1() {
        let mut hands = parse_input(TEST_INPUT);
        hands.sort();
        let winnings: i32 = hands
            .iter()
            .zip(1..=hands.len())
            .map(|(hand, rank)| hand.bid * rank as i32)
            .sum();

        assert_eq!(winnings, 6440)
    }

    #[test]
    fn test_input_pt2() {
        let mut hands = parse_input(TEST_INPUT)
            .iter_mut()
            .map(|hand| hand.upgrade())
            .collect::<Vec<Hand>>();
        hands.sort();
        let winnings: i32 = hands
            .iter()
            .zip(1..=hands.len())
            .map(|(hand, rank)| hand.bid * rank as i32)
            .sum();

        assert_eq!(winnings, 5905)
    }

    #[test]
    fn sorting() {
        let mut test_hands = TEST_HANDS.clone();
        test_hands.sort();
        test_hands
            .iter()
            .zip(BID_ORDER.iter())
            .for_each(|(input, &test)| assert_eq!(input.bid, test))
    }

    #[test]
    fn parsing() {
        parse_input(TEST_INPUT)
            .iter()
            .zip(TEST_HANDS.iter())
            .for_each(|(input, test)| assert_eq!(input, test))
    }

    const BID_ORDER: [i32; 5] = [765, 220, 28, 684, 483]; // used to test if sorted properly

    const TEST_HANDS: [Hand; 5] = [
        Hand {
            bid: 765,
            cards: [3, 2, 10, 3, 13],
            hand_type: Type::OnePair,
        },
        Hand {
            bid: 684,
            cards: [10, 5, 5, 11, 5],
            hand_type: Type::ThreeKind,
        },
        Hand {
            bid: 28,
            cards: [13, 13, 6, 7, 7],
            hand_type: Type::TwoPair,
        },
        Hand {
            bid: 220,
            cards: [13, 10, 11, 11, 10],
            hand_type: Type::TwoPair,
        },
        Hand {
            bid: 483,
            cards: [12, 12, 12, 11, 14],
            hand_type: Type::ThreeKind,
        },
    ];

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
}
