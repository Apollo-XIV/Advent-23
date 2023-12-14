
fn main() {
    let inputs: u32  = read_lines("data/day_02.txt")
        .into_iter()
        .map(|line| parse_game(line))
        .map(|game| find_power(game))
        .sum();
    println!("{:?}", inputs); 
    // parse inputs into games
    // filter inputs where values are greater than (12, 13, 14)

}

#[derive(Debug)]
struct Game {
    id: u32,
    colours: [u8; 3]
}

fn find_power(game: Game) -> u32 {
    game.colours.into_iter().map(u32::from).product()
}

fn parse_game(input: String) -> Game {
    let split_games = input.split(":");
    let mut game = Game {id: 0, colours: [0,0,0]};
    // get game id
    game.id = split_games.to_owned().nth(0).unwrap().split(" ").nth(1).unwrap().parse().unwrap();
    println!("{}",game.id);
    // parse rounds
    let mut rounds = split_games.to_owned().nth(1).unwrap().split(";");
    for round in rounds {
        // parse cubes
        let cubes: Vec<&str> = round.split(", ").collect();
        for cube in cubes {
            let mut cube: Vec<&str>   = cube.split(" ").collect();
            if cube.len() > 2 {
                cube.remove(0);
            }
            println!("{:?}",cube);
            match cube[1] {
                "red" if game.colours[0] < cube.to_owned()[0].parse::<u8>().unwrap() => game.colours[0] = cube.to_owned()[0].parse::<u8>().unwrap(),
                "green" if game.colours[1] < cube.to_owned()[0].parse::<u8>().unwrap() => game.colours[1] = cube.to_owned()[0].parse::<u8>().unwrap(),
                "blue" if game.colours[2] < cube.to_owned()[0].parse::<u8>().unwrap() => game.colours[2] = cube.to_owned()[0].parse::<u8>().unwrap(),
                _ => continue
            };
            // let ()
            // match cube.nth() {}
        }
    }
    println!("{:?}", game.colours);
    game
}

fn validate_bags(input: [u8; 3], screen: [u8; 3]) -> bool {
     match input
        .iter()
        .zip(screen.iter())
        .map(|(a, b)| a <= b)
        .reduce(|acc, e| acc && e) {
            Some(x) => x,
            None => false
        }
    
}

use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}