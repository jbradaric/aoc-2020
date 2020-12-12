use std::{num::ParseIntError, str::FromStr};

const INPUT: &str = include_str!("../input");

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    North,
    South,
    East,
    West,
    Forward,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Move(Move, i32),
    Turn(Turn, u32),
}

#[derive(Debug)]
struct ParseError {}

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        ParseError {}
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('N') => Ok(Instruction::Move(Move::North, s.split_at(1).1.parse()?)),
            Some('S') => Ok(Instruction::Move(Move::South, s.split_at(1).1.parse()?)),
            Some('E') => Ok(Instruction::Move(Move::East, s.split_at(1).1.parse()?)),
            Some('W') => Ok(Instruction::Move(Move::West, s.split_at(1).1.parse()?)),
            Some('F') => Ok(Instruction::Move(Move::Forward, s.split_at(1).1.parse()?)),
            Some('L') => Ok(Instruction::Turn(Turn::Left, s.split_at(1).1.parse()?)),
            Some('R') => Ok(Instruction::Turn(Turn::Right, s.split_at(1).1.parse()?)),
            _ => Err(ParseError {}),
        }
    }
}

struct Ship {
    pub direction: Move,
    pub x: i32,
    pub y: i32,
}

impl Ship {
    fn move_ship(&mut self, inst: Instruction) {
        match inst {
            Instruction::Move(m, n) => {
                self.do_move(m, n);
            }
            Instruction::Turn(t, n) => {
                self.do_turn(t, n);
            }
        };
    }

    fn do_move(&mut self, m: Move, distance: i32) {
        match m {
            Move::North => {
                self.y += distance;
            }
            Move::South => {
                self.y -= distance;
            }
            Move::East => {
                self.x += distance;
            }
            Move::West => {
                self.x -= distance;
            }
            Move::Forward => {
                self.do_move(self.direction, distance);
            }
        }
    }

    fn do_turn(&mut self, t: Turn, deg: u32) {
        const RIGHT_TURNS: [Move; 4] = [Move::North, Move::East, Move::South, Move::West];
        const LEFT_TURNS: [Move; 4] = [Move::North, Move::West, Move::South, Move::East];
        let num: usize = deg as usize / 90;
        let new_d = match t {
            Turn::Left => LEFT_TURNS
                .iter()
                .cycle()
                .skip_while(|x| **x != self.direction)
                .skip(num)
                .next()
                .unwrap(),
            Turn::Right => RIGHT_TURNS
                .iter()
                .cycle()
                .skip_while(|x| **x != self.direction)
                .skip(num)
                .next()
                .unwrap(),
        };
        self.direction = *new_d;
    }

    fn move_to_waypoint(&mut self, w: &Waypoint, n: i32) {
        self.x += n * w.x;
        self.y += n * w.y;
    }
}

struct Waypoint {
    pub x: i32,
    pub y: i32,
}

impl Waypoint {
    fn move_to(&mut self, m: Move, d: i32) {
        match m {
            Move::North => {
                self.y += d;
            }
            Move::South => {
                self.y -= d;
            }
            Move::East => {
                self.x += d;
            }
            Move::West => {
                self.x -= d;
            }
            Move::Forward => unreachable!(),
        }
    }

    fn rotate(&mut self, dir: Turn, angle: u32) {
        let (rx, ry) = match dir {
            Turn::Left => (
                self.x * int_cos(angle) - self.y * int_sin(angle),
                self.x * int_sin(angle) + self.y * int_cos(angle),
            ),
            Turn::Right => (
                self.x * int_cos(angle) + self.y * int_sin(angle),
                -1 * self.x * int_sin(angle) + self.y * int_cos(angle),
            ),
        };
        self.x = rx;
        self.y = ry;
    }
}

fn main() {
    let mut ship = Ship {
        direction: Move::East,
        x: 0,
        y: 0,
    };
    for instruction in INPUT
        .trim()
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
    {
        ship.move_ship(instruction);
    }
    println!("Distance: {}", ship.x.abs() + ship.y.abs());

    let mut ship = Ship {
        direction: Move::East,
        x: 0,
        y: 0,
    };
    let mut waypoint = Waypoint { x: 10, y: 1 };
    for instruction in INPUT
        .trim()
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
    {
        match instruction {
            Instruction::Move(Move::Forward, n) => {
                ship.move_to_waypoint(&waypoint, n);
            }
            Instruction::Move(m, n) => {
                waypoint.move_to(m, n);
            }
            Instruction::Turn(a, d) => {
                waypoint.rotate(a, d);
            }
        };
    }
    println!("Distance 2: {}", ship.x.abs() + ship.y.abs());
}

fn int_cos(angle: u32) -> i32 {
    let abs_angle = angle % 360;
    match abs_angle / 90 {
        0 => 1,
        1 => 0,
        2 => -1,
        3 => 0,
        _ => unreachable!(),
    }
}

fn int_sin(angle: u32) -> i32 {
    let abs_angle = angle % 360;
    match abs_angle / 90 {
        0 => 0,
        1 => 1,
        2 => 0,
        3 => -1,
        _ => unreachable!(),
    }
}
