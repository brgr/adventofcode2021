use crate::Direction::{DOWN, FORWARD, UP};
use inputs;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug)]
enum Direction {
    FORWARD,
    DOWN,
    UP,
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "forward" => FORWARD,
            "down" => DOWN,
            "up" => UP,
            _ => panic!(),
        })
    }
}

#[derive(Debug)]
struct Movement {
    direction: Direction,
    steps: u16,
}

impl FromStr for Movement {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss: Vec<&str> = s.split(' ').collect();

        let direction = Direction::from_str(ss[0])?;
        let steps = ss[1].parse::<u16>().unwrap();

        Ok(Movement { direction, steps })
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

fn solve_part1(ms: Vec<Movement>) -> Position {
    let mut pos = Position { x: 0, y: 0, aim: 0 };

    for m in ms {
        match m.direction {
            FORWARD => pos.x += (m.steps as i32),
            DOWN => pos.y += (m.steps as i32),
            UP => pos.y -= (m.steps as i32),
        }
    }

    pos
}

fn solve_part2(ms: Vec<Movement>) -> Position {
    let mut pos = Position { x: 0, y: 0, aim: 0 };

    for m in ms {
        match m.direction {
            FORWARD => {
                pos.x += (m.steps as i32);
                pos.y += (pos.aim * m.steps as i32);
            }
            DOWN => pos.aim += (m.steps as i32),
            UP => pos.aim -= (m.steps as i32),
        }
    }

    pos
}

fn main() {
    let inputs: Vec<Movement> = inputs::get_input_split(2)
        .iter()
        .map(|i| Movement::from_str(i).unwrap())
        .collect();

    println!("{:?}", inputs);
    let position = solve_part2(inputs);
    println!("{:?}", position);

    println!("{}", (position.x * position.y));
}
