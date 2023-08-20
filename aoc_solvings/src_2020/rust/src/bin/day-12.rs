use derive_more::{Add, Sub};
use itertools::Itertools;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_str = get_input(&args[1]);
    let inputs = parse_input(&input_str);
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
}

fn get_input(input_file: &String) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

fn parse_input(input: &str) -> Vec<ShipInstruction> {
    let degree_per_turn: isize = 90;
    input
        .lines()
        .map(|l| {
            let command = l.as_bytes()[0];
            let value: isize = l[1..].parse().unwrap();
            match command {
                b'N' => ShipInstruction::Move(Direction::North, value),
                b'E' => ShipInstruction::Move(Direction::East, value),
                b'S' => ShipInstruction::Move(Direction::South, value),
                b'W' => ShipInstruction::Move(Direction::West, value),
                b'F' => ShipInstruction::Forward(value),
                b'L' => ShipInstruction::Rotate(DeltaAngle(-value / degree_per_turn)),
                b'R' => ShipInstruction::Rotate(DeltaAngle(value / degree_per_turn)),
                c => panic!("Unexpected instruction type: {}", c as char),
            }
        })
        .collect_vec()
}

fn part1(instructions: &[ShipInstruction]) -> usize {
    let start_ship = Ship::default();
    // implement Add for Ship, we can fold each instruction into the initial ship state
    let end_ship = instructions
        .iter()
        .fold(start_ship, |curr_ship, &instr| curr_ship + instr);
    end_ship.pos.manhattan_distance()
}

fn part2(instructions: &[ShipInstruction]) -> usize {
    let start_ship = Ship::default();
    let end_ship = instructions.iter().fold(start_ship, |curr_ship, &instr| {
        curr_ship.waypoint_instructions(instr)
    });
    end_ship.pos.manhattan_distance()
}

const DIR_MAX: isize = 4;

#[derive(Debug, Copy, Clone)]
enum ShipInstruction {
    // given instruction set won't contain negatives values
    // absolute directional move
    Move(Direction, isize),
    // directional move
    Forward(isize),
    // rotate
    Rotate(DeltaAngle),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Add, Sub)]
struct Position {
    // represent 2D position
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DeltaAngle(isize);

#[derive(Clone, Copy, Debug)]
struct Ship {
    direction: Direction,
    pos: Position,
    waypoint: Position,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            direction: Direction::East,
            pos: Position { x: 0, y: 0 },
            waypoint: Position { x: 10, y: 1 },
        }
    }
}

impl TryFrom<isize> for Direction {
    type Error = &'static str;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if (0..=3).contains(&value) {
            Ok(unsafe { std::mem::transmute(u8::try_from(value).unwrap()) })
        } else {
            Err("Direction out of bounds.")
        }
    }
}

impl From<Direction> for isize {
    fn from(val: Direction) -> Self {
        val as _
    }
}

impl std::ops::Add<DeltaAngle> for Direction {
    type Output = Self;
    fn add(self, rhs: DeltaAngle) -> Self::Output {
        let curr_dir: isize = self.into();
        (curr_dir + rhs.0).rem_euclid(DIR_MAX).try_into().unwrap()
    }
}

impl Direction {
    const fn position_vec(self) -> Position {
        match self {
            Self::North => Position { x: 0, y: 1 },
            Self::East => Position { x: 1, y: 0 },
            Self::South => Position { x: 0, y: -1 },
            Self::West => Position { x: -1, y: 0 },
        }
    }
}

impl std::ops::Mul<isize> for Position {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Position {
    fn manhattan_distance(&self) -> usize {
        // assumption that vector goes from 0, 0 to this Position
        (self.x.abs() + self.y.abs()).try_into().unwrap()
    }
    fn rotate(self, angle: DeltaAngle) -> Self {
        let Self { x, y } = self;
        match angle.0.rem_euclid(DIR_MAX) {
            0 => Self { x, y },
            1 => Self { x: y, y: -x },
            2 => Self { x: -x, y: -y },
            3 => Self { x: -y, y: x },
            _ => unreachable!(),
        }
    }
}

impl std::ops::Add<ShipInstruction> for Ship {
    type Output = Self;
    fn add(self, rhs: ShipInstruction) -> Self::Output {
        match rhs {
            ShipInstruction::Move(dir, val) => Self {
                pos: self.pos + dir.position_vec() * val,
                ..self
            },
            ShipInstruction::Forward(val) => Self {
                pos: self.pos + self.direction.position_vec() * val,
                ..self
            },
            ShipInstruction::Rotate(angle) => Self {
                direction: self.direction + angle,
                ..self
            },
        }
    }
}

impl Ship {
    fn waypoint_instructions(self, rhs: ShipInstruction) -> Self {
        match rhs {
            ShipInstruction::Move(dir, val) => Self {
                waypoint: self.waypoint + dir.position_vec() * val,
                ..self
            },
            ShipInstruction::Forward(val) => Self {
                pos: self.pos + self.waypoint * val,
                ..self
            },
            ShipInstruction::Rotate(angle) => Self {
                waypoint: self.waypoint.rotate(angle),
                ..self
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "inputs/test-12.txt";

    #[test]
    fn test_position_add() {
        let p1 = Position { x: 1, y: 1 };
        let p2 = Position { x: 3, y: 2 };
        assert_eq!(p1 + p2, Position { x: 4, y: 3 });
    }

    #[test]
    fn test_position_sub() {
        let p1 = Position { x: 1, y: 1 };
        let p2 = Position { x: 3, y: 2 };
        assert_eq!(p1 - p2, Position { x: -2, y: -1 });
    }

    #[test]
    fn test_manhattan_dist() {
        let start = Position { x: 0, y: 0 };
        let end = Position { x: 17, y: -8 };
        assert_eq!((end - start).manhattan_distance(), 25);
    }

    #[test]
    fn direction_try_from() {
        use std::convert::TryFrom;

        assert_eq!(
            <Direction as TryFrom<isize>>::try_from(0).unwrap(),
            Direction::North
        );
        assert_eq!(
            <Direction as TryFrom<isize>>::try_from(2).unwrap(),
            Direction::South
        );
        assert!(<Direction as TryFrom<isize>>::try_from(-1).is_err(),);
        assert!(<Direction as TryFrom<isize>>::try_from(4).is_err(),);
    }

    #[test]
    fn test_shipdirection_into() {
        let x: isize = 3;
        assert_eq!(TryInto::<Direction>::try_into(x), Ok(Direction::West));
    }

    #[test]
    fn test_shipdirection_incr() {
        assert_eq!(Direction::East + DeltaAngle(1), Direction::South);
        assert_eq!(Direction::East + DeltaAngle(2), Direction::West);
    }

    #[test]
    fn test_shipdirection_wrap() {
        assert_eq!(Direction::East + DeltaAngle(4), Direction::East);
        assert_eq!(Direction::West + DeltaAngle(2), Direction::East);
    }

    #[test]
    fn test_shipdirection_decr_wrap() {
        assert_eq!(Direction::East + DeltaAngle(-2), Direction::West);
    }

    #[test]
    fn test_position_rotate() {
        let v = Position { x: 3, y: 1 };
        assert_eq!(v.rotate(DeltaAngle(0)), v);
        assert_eq!(v.rotate(DeltaAngle(4)), v);
        assert_eq!(v.rotate(DeltaAngle(-4)), v);

        assert_eq!(v.rotate(DeltaAngle(1)), Position { x: 1, y: -3 });
        assert_eq!(v.rotate(DeltaAngle(2)), Position { x: -3, y: -1 });
        assert_eq!(v.rotate(DeltaAngle(3)), Position { x: -1, y: 3 });
    }

    #[test]
    fn correct_part1() {
        let input = parse_input(&get_input(&TEST_FILE.to_string()));

        assert_eq!(part1(&input), 25);
    }

    #[test]
    fn correct_part2() {
        let input = parse_input(&get_input(&TEST_FILE.to_string()));

        assert_eq!(part2(&input), 286);
    }
}
