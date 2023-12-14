
fn main() {
    let mut board: Board = Board { grid: into_array(read_lines("data/day_03.txt")
        .into_iter()
        .map(|x| parse_row(x))
        .collect())
    };

    // println!("{:?}", board.grid[139].get(1..90));
    
    board.step();
    board.step();
    board.step();
    board.step();
    board.step();
    board.step();
    board.cull_part_digits(); // evaluate adjacent parts and combine
    let parts: Vec<u32> = board.get_part_numbers().into_iter().collect();
    println!("{:?}", parts.len() );
    // board.grid.into_iter()
    //     .map(|x| x.map(|y| match y {
    //         State::Part(x) => '_'.to_string(),
    //         State::Digit(x) => x.to_string(),
    //         State::Symbol => '*'.to_string(),
    //         State::Blank => '.'.to_string()
    //     }).join(""))
    //     .for_each(|z| println!("{:?}", z )); 
}

#[derive(Debug)]
enum State {
    Part(u32),
    Digit(u32),
    Symbol,
    Blank
}

impl State {
    fn activate(&mut self) {
        match &self {
            State::Digit(x) => *self = State::Part(*x),
            _ => ()
        }
    }
}

struct Board {
    grid: [[State; 140]; 140]
}
#[derive(Clone, Copy)]
struct Point{x: u32,y: u32}
impl Point {
    fn destructure(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }
}

struct IntPoint{x: i32,y: i32}
impl IntPoint {
    fn to_point(&self) -> Point {
        Point {x: self.x as u32, y: self.y as u32}
    }
}

impl Board {

    fn step(&mut self) {
        let points: Vec<Point> = (0..139).flat_map(|y| (0..139).map(move |x| Point {x, y})).collect(); 
        for point in points {
        // self.set(&point, State::Blank);
            match self.find(&point) {
                Some(State::Symbol) => self
                    .get_kernel(&point).iter()
                    .for_each(|target| {
                        self.find(target).expect("Point in Vec was outside grid bounds").activate()
                    }),
                Some(State::Part(_)) => self
                    .get_neighbours(&point).iter()
                    .for_each(|neighbour| {
                        self.find(neighbour).expect("Point in vec was outside grid bounds").activate()
                    }),
                Some(State::Blank | State::Digit(_)) => continue,
                None => continue
            };
        }
    }

    pub fn cull_part_digits(&mut self) {
        let points: Vec<Point> = (0..140).flat_map(|y| (0..139).map(move |x| Point {x, y})).collect(); 
        for point in points {
            let mut neighbours: Vec<Point> = vec![];
            let mut value: u32 = 0;
        // self.set(&point, State::Blank);
            match (self).find(&point) {
                Some(State::Part(subject)) => {
                    value = *subject;
                    self
                        .get_rightneighbour(&point)
                        .iter()
                        .for_each(|z| neighbours.push(*z));
                    
                },
                _ => continue
            };
            neighbours.iter().for_each(|neighbour| {
                match self.find(neighbour) {
                    Some(State::Part(target)) => {
                        // combine digits target and subject into one value (order of target then subject)
                        let a = value as i32;
                        let b = target.to_owned() as i32;
        
                        let out = concat_new(&[a, b]);
        
        
                        self.set(&neighbour, State::Part(out as u32));
                        self.set(&point, State::Blank);

                    },
                    _ => ()
                }
            });
        }
    }

    fn get_part_numbers(&mut self) -> Vec<u32> {
        let mut output: Vec<u32> = vec![];
        let points: Vec<Point> = (0..140).flat_map(|y| (0..139).map(move |x| Point {x, y})).collect(); 
        for point in points {
            match self.find(&point) {
                Some(State::Part(x)) => output.push(x.to_owned() as u32),
                _ => continue
            }
        }
        output
    }


    pub fn set(&mut self, point: &Point, state: State) {
        if point.x >= 140 || point.y >= 140 { return };
        self.grid[point.y as usize][point.x as usize] = state; 
    }

    pub fn find(&mut self, point: &Point) -> Option<&mut State> {
        if point.x >= 140 || point.y >= 140 {
            None
        } else {
            Some(&mut self.grid[point.y as usize][point.x as usize])
        }
    }

    fn get_neighbours(&mut self, bullseye: &Point) -> Vec<Point> {
        let (x, y) = bullseye.destructure();
        let targets = vec![IntPoint{x: x-1, y}, IntPoint{x: x+1, y}];
        let mut output: Vec<Point> = vec![];
        for point in targets {
            match self.find(&point.to_point()) {
                Some(_) => output.push(point.to_point()),
                None => continue
            }
        }
        output

    }

    fn get_rightneighbour(&mut self, bullseye: &Point) -> Vec<Point> {
        let (x, y) = bullseye.destructure();
        match self.find(&IntPoint{x: x+1, y}.to_point()) {
            Some(_) => vec![IntPoint{x: x+1, y}.to_point()],
            None => vec![]
        }
    }

    fn get_kernel(&mut self, bullseye: &Point) -> Vec<Point> { // used to observe the points around a given point. Returns an <= 8 long vector containing references to the neighbors
        let (x, y) = bullseye.destructure();
        let targets = [
            [IntPoint{x: x - 1, y: y - 1}, IntPoint{x: x, y: y - 1}, IntPoint{x: x + 1,y: y - 1}],
            [IntPoint{x: x - 1, y: y}, IntPoint{x: x, y: y}, IntPoint{x: x + 1,y: y}],
            [IntPoint{x: x - 1, y: y + 1}, IntPoint{x: x, y: y + 1}, IntPoint{x: x + 1,y: y + 1}] 
        ];
        let mut output: Vec<Point> = vec![];
        for set in targets {
            for point in set {
                match self.find(&point.to_point()) {
                    Some(_) => output.push(point.to_point()),
                    None => continue
                }
            }
        }
        output
    }
}

fn parse_row(input: String) -> [State; 140] {
    let char_vec = input
        .chars()
        .map(|x| parse_digit(x))
        .collect();


    into_array(char_vec)
}

fn parse_digit(input: char) -> State {
    match input as u8 {
        x if x == 46 => State::Blank,
        x if x >= 48 && x <= 57 => State::Digit((x as char).to_digit(10).unwrap() as u32),
        _ => State::Symbol
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

use std::convert::TryInto;

fn into_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

fn concat_new(vec: &[i32]) -> i32 {
    let t = vec.iter().fold("".to_string(), |acc, x| acc + &x.to_string());
    t.parse::<i32>().unwrap()
}
