use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    iter::{self, Cycle},
    slice::Iter,
};

// fn part_1() {
//     let (map, directions) = parse_input(&read_to_string("data/day_08.txt").unwrap());
//     let mut directions = directions.iter().cycle();
//     let mut next = "AAA".to_string();
//     let mut steps = 0;
//     while next != "ZZZ".to_string() {
//         let node = map.get(&next).unwrap();
//         next = node.go(directions.next().unwrap());
//         steps += 1;
//     }
//     println!("{:?}", steps);
// }

fn part_1() -> i64 {
    let (map, directions) = parse_input(&read_to_string("data/day_08.txt").unwrap());
    let mut directions = directions.iter().cycle();
    let mut queue = VecDeque::from([("AAA".to_string(), 0)]);
    let mut step = 0;
    let mut direction = directions.next().unwrap();
    let mut out = 0;
    while let Some((key, steps)) = queue.pop_front() {
        // dbg!(&queue);
        // println!("key {:?}\t | steps: {:?}\t | step: {:?}", key, steps, step);
        if key.ends_with('Z') {
            out = steps;
            break;
        }
        if step != steps {
            direction = directions.next().unwrap();
            step = steps
        } // advance the iterator if we've processed all the
        let next_key = map.get(&key).unwrap().go(&direction);
        queue.push_back((next_key, steps + 1))
    }
    out
}

fn part_2() -> i64 {
    let (map, directions) = parse_input(&read_to_string("data/day_08.txt").unwrap());
    let mut directions = directions.iter().cycle();
    let mut queue: VecDeque<(String, i64)> = map
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| x.to_owned())
        .zip(iter::repeat(0 as i64))
        .collect();
    let mut step = 0;
    let mut direction = directions.next().unwrap();
    let mut out = vec![];
    while let Some((key, steps)) = queue.pop_front() {
        if key.ends_with('Z') {
            out.push(steps);
            continue;
        }
        if step != steps {
            direction = directions.next().unwrap();
            step = steps
        } // advance the iterator if we've processed all the
        let next_key = map.get(&key).unwrap().go(&direction);
        queue.push_back((next_key, steps + 1))
    }
    use num::integer::lcm;
    out.into_iter().reduce(|acc, e| lcm(acc, e)).unwrap()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Node(String, String);
impl Node {
    fn go(&self, direction: &Direction) -> String {
        match direction {
            Direction::Left => self.0.clone(),
            Direction::Right => self.1.clone(),
        }
    }
}

fn parse_input(input: &str) -> (HashMap<String, Node>, Vec<Direction>) {
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut lines = input.split('\n');
    let directions: Vec<Direction> = lines
        .next()
        .unwrap()
        .chars()
        .filter_map(|char| match char {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        })
        .collect();

    lines.skip(1).for_each(|line| {
        let [key, value]: [&str; 2] = line.split(" = ").collect::<Vec<&str>>().try_into().unwrap();
        let key = key.to_string();
        let value = Node(value[1..4].to_string(), value[6..9].to_string());
        nodes.insert(key, value);
    });

    (nodes, directions)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(), 14299763833181)
    }
    #[test]
    fn part_1_test() {
        assert_eq!(part_1(), 18157)
    }

    #[test]
    fn test_input_pt2() {
        let (map, directions) = parse_input(TEST_INPUT_2);
        let mut directions = directions.iter().cycle();
        let mut queue: VecDeque<(String, i64)> = map
            .keys()
            .filter(|x| x.ends_with('A'))
            .map(|x| x.to_owned())
            .zip(iter::repeat(0 as i64))
            .collect();
        let mut step = 0;
        let mut direction = directions.next().unwrap();
        let mut out = vec![];
        while let Some((key, steps)) = queue.pop_front() {
            dbg!(&queue);
            println!("key {:?}\t | steps: {:?}\t | step: {:?}", key, steps, step);
            if key.ends_with('Z') || steps == 10 {
                out.push(steps);
                continue;
            }
            if step != steps {
                direction = directions.next().unwrap();
                step = steps
            } // advance the iterator if we've processed all the
            let next_key = map.get(&key).unwrap().go(&direction);
            queue.push_back((next_key, steps + 1))
        }
        use num::integer::lcm;
        println!("{:?}", out.into_iter().reduce(|acc, e| lcm(acc, e)));
    }

    #[test]
    fn test_input_pt1() {
        let (map, directions) = parse_input(TEST_INPUT);
        let mut directions = directions.iter().cycle();
        let mut queue = VecDeque::from([("AAA".to_string(), 0)]);
        let mut step = 0;
        let mut direction = directions.next().unwrap();
        let mut out = 0;
        while let Some((key, steps)) = queue.pop_front() {
            dbg!(&queue);
            println!("key {:?}\t | steps: {:?}\t | step: {:?}", key, steps, step);
            if key.ends_with('Z') || steps == 10 {
                out = steps;
                break;
            }
            if step != steps {
                direction = directions.next().unwrap();
                step = steps
            } // advance the iterator if we've processed all the
            let next_key = map.get(&key).unwrap().go(&direction);
            queue.push_back((next_key, steps + 1))
        }
        println!("{:?}", out);
        assert_eq!(2, out);
    }

    #[test]
    fn parsing() {
        println!("test");

        let (map, directions) = parse_input(TEST_INPUT);
        TEST_DIRECTIONS
            .into_iter()
            .zip(directions)
            .for_each(|(test, input)| assert_eq!(test, input));
        let mut keys: Vec<String> = map.keys().map(|string| string.clone()).collect();
        keys.sort();
        keys.iter()
            .zip(TEST_KEYS)
            .for_each(|(input, test)| assert_eq!(input, &test));
    }

    const TEST_KEYS: [&str; 7] = ["AAA", "BBB", "CCC", "DDD", "EEE", "GGG", "ZZZ"];

    const TEST_DIRECTIONS: [Direction; 2] = [Direction::Right, Direction::Left];

    const TEST_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
}
