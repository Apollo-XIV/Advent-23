
fn main() {
    let board = Schematic { grid: read_lines("data/day_03.txt")
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect()
    };

    let numbers: Vec<Number> = board.find_numbers()
        .into_iter()
        .filter(|number| number.is_part(&board))
        .collect();
    // println!("{:?}",board.get(Point(0,20)));
    // let number1 = Number(Point(0,20),Point(2,20));
    // let number2 = Number(Point(12,0),Point(15,0));
    // let number3 = Number(Point(13,2),Point(16,2));
    // let number4 = Number(Point(1,13),Point(2,13));
    // numbers[numbers.len()-2].describe(&board);
    // let number = numbers.get(0).expect("empty :(");
    // let row: Vec<char> = "............830..743.......59..955.......663......".chars().collect();
    // number.value(&board);
    // numbers.iter().for_each(|&x| {println!("{:?}",x.value(&board));});
    let sum: i32 = numbers.iter().map(|x| x.value(&board)).sum();
    println!("{:?}", sum);
    // println!("{:?}", numbers);
    // (12..15).into_iter().for_each(|x| println!("{:?}",board.get(Point(x,0))));

}

const GRID_X: i32 = 140;
const GRID_Y: i32 = 140;
const INVALID_POINT_ERR: &str = "Invalid point marked as valid.";

struct Schematic {
    grid: Vec<Vec<char>>
} 
impl Schematic {
    fn get(&self, point: Point) -> Option<&char> {
        if point.is_valid() {
            Some(&self.grid
                .get(point.1 as usize)
                .expect(INVALID_POINT_ERR)
                .get(point.0 as usize)
                .expect(INVALID_POINT_ERR))
        } else {
            None
        }
    }

    fn find_numbers(&self) -> Vec<Number> {
        let mut output: Vec<Number> = vec![];
        for y in 0..(GRID_Y) {
            parse_row(&self.grid.get(y as usize).expect(INVALID_POINT_ERR), y)
                .iter()
                .for_each(|&z| output.push(z));
        }
        output
    }
}

fn parse_row(row: &Vec<char>, y: i32) -> Vec<Number> {
    let mut output: Vec<Number> = vec![];
    let mut start = Point(0,0);
    let mut end = Point(0,0);
    let mut tracking: bool = false;
    for x in 0..(GRID_X+10) {
        match row.get(x as usize) {
            Some(&c) if ((c as u8) >= 48) && ((c as u8) < 58) && tracking == false => {
                start = Point(x,y);
                tracking = true;
            },
            Some(&c) if ((c as u8) >= 48) && ((c as u8) < 58) && tracking == true => (),
            Some(_) | None if tracking == true => {
                end = Point(x-1,y);
                output.push(Number(start,end));
                tracking = false;
            }, // push completed 
            _ => ()
        }
    }
    output
}

#[derive(Debug, Clone, Copy)]
struct Number(Point,Point);
impl Number {
    fn describe(&self, schematic: &Schematic) {
        println!("loc: {:?}", self);
        println!("value: {:?}", self.value(&schematic));
        println!("kernel: {:?}", self.get_kernel());
    }

    fn get_kernel(&self) -> Vec<Point> {
        // generate vec of points for the surrounding mask
        let start = self.0;
        let end = self.1;
        let mut top_row: Vec<Point> = (start.0-1..=end.0+1)
            .into_iter()
            .map(|i| Point(i, start.1+1))
            .collect();
        let mut bottom_row: Vec<Point> = (start.0-1..=end.0+1)
            .into_iter()
            .map(|i| Point(i, start.1-1))
            .collect();
        top_row.append(&mut bottom_row);
        top_row.append(&mut vec![Point(start.0-1, start.1),Point(end.0+1, end.1)]);
        // println!("{:?}", self);
        top_row
            .into_iter()
            .filter(|&point| point.is_valid())
            .collect()
    }

    fn value(&self, schematic: &Schematic) -> i32 {
        let start = self.0;
        let end  = self.1;
        schematic.grid[start.1 as usize][(start.0 as usize)..=(end.0 as usize)]
            .into_iter()
            .collect::<String>()
            .parse::<i32>()
            .expect("this is the fuck")
    }

    fn is_part(&self, schematic: &Schematic) -> bool {
        self.get_kernel()
            .iter()
            .any(|&point| match schematic.get(point) {
                Some(&c) if ((c as u8) >= 48) && ((c as u8) < 58) || c == '.' => {
                    println!("{:?}", c);
                    false
                },
                None => false,
                _ => true
            })
    }
}

#[derive(Debug, Clone, Copy)]
struct Point(i32, i32);
impl Point {
    fn is_valid(&self) -> bool {
        self.0 >= 0 && self.0 < GRID_X && self.1 >= 0 && self.1 < GRID_Y
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
