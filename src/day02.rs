use regex::Regex;
use std::assert;
use std::fs;
use std::ops;
use std::string::ParseError;

enum Direction {
    FORWARD,
    DOWN,
    UP,
}

struct Move {
    pub direction: Direction,
    pub distance: u16,
}

struct Point {
    pub x: i32,
    pub y: i32,
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct MoveState {
    pub x: i32,
    pub y: i32,
    pub aim: i32
}

impl Move {
    fn point(self: &Self) -> Point {
        match self.direction {
            Direction::FORWARD => Point {
                x: self.distance as i32,
                y: 0,
            },
            Direction::DOWN => Point {
                x: 0,
                y: self.distance as i32,
            },
            Direction::UP => Point {
                x: 0,
                y: -(self.distance as i32),
            },
        }
    }
    fn update_state(self: &Self, state: &MoveState) -> MoveState {
        match self.direction {
            Direction::FORWARD => MoveState{
                x: state.x + self.distance as i32,
                y: state.y + state.aim * self.distance as i32,
                aim: state.aim
            },
            Direction::DOWN => MoveState{
                x: state.x,
                y:state.y,
                aim: state.aim + self.distance as i32
            },
            Direction::UP => MoveState {
                x: state.x,
                y:state.y,
                aim: state.aim - self.distance as i32
            }
        }
    }
}

impl std::str::FromStr for Move {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new("(forward|down|up) (\\d+)").unwrap();
        let cap = re.captures(s).unwrap();
        let direction = match &cap[1] {
            "forward" => Direction::FORWARD,
            "down" => Direction::DOWN,
            "up" => Direction::UP,
            _ => Direction::FORWARD,
        };
        let distance = &cap[2].parse::<u16>().unwrap();
        Ok(Move {
            direction: direction,
            distance: *distance,
        })
    }
}

fn part1(name: &str) -> i32 {
    let f = fs::read_to_string(name)
        .expect("Couldn't open file")
        .split("\r\n")
        .filter(|s| s.len() != 0)
        .map(|s| s.parse::<Move>().unwrap().point())
        .fold(Point{x:0,y:0}, |acc,p| acc + p);
    f.x*f.y
}

fn part2(name: &str) -> i32 {
    let f = fs::read_to_string(name)
        .expect("Couldn't open file")
        .split("\r\n")
        .filter(|s| s.len() != 0)
        .map(|s| s.parse::<Move>().unwrap())
        .fold(MoveState{x:0,y:0,aim:0}, |acc,p| p.update_state(&acc));
    f.x*f.y
}

fn main() {
    assert!(
        part1("data/Day02_test.txt") == 150,
        "Part 1 failed validation"
    );
    println!("Part 1: {}", part1("data/Day02.txt"));
    assert!(part2("data/Day02_test.txt") == 900, "Part 2 failed validation");
    println!("Part 2: {}", part2("data/Day02.txt"));
}
