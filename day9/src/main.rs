use std::collections::HashSet;
use std::fs;
use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, PartialEq)]
enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Move {
    fn to_position_offset(&self) -> Position {
        match self {
            Move::Up(_) => Position { x: 0, y: 1 },
            Move::Down(_) => Position { x: 0, y: -1 },
            Move::Left(_) => Position { x: -1, y: 0 },
            Move::Right(_) => Position { x: 1, y: 0 },
        }
    }

    fn move_quantity(&self) -> i32 {
        match self {
            Move::Up(i) => *i,
            Move::Down(i) => *i,
            Move::Left(i) => *i,
            Move::Right(i) => *i,
        }
    }
}

impl TryFrom<String> for Move {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut split = value.split_whitespace();
        let move_direction = split.next().unwrap();
        let move_quantity = split
            .next()
            .unwrap()
            .parse::<i32>()
            .map_err(|_| ())
            .unwrap();

        match move_direction {
            "R" => Ok(Move::Right(move_quantity)),
            "L" => Ok(Move::Left(move_quantity)),
            "U" => Ok(Move::Up(move_quantity)),
            "D" => Ok(Move::Down(move_quantity)),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Bridge {
    positions: Vec<Position>,
    visited: HashSet<Position>,
}

impl Bridge {
    fn new(num: usize) -> Self {
        let mut ret = Bridge {
            positions: vec![Position { x: 0, y: 0 }; num],
            visited: HashSet::new(),
        };
        ret.visited.insert(*ret.positions.iter().last().unwrap());
        ret
    }

    fn calc_move_tail(head: &Position, tail: &Position) -> Position {
        if head.adjacent(tail) || tail == head {
            return Position { x: 0, y: 0 };
        }

        let diff = *head - *tail;
        let mut x_mult = 1;
        let mut y_mult = 1;
        if diff.x < 0 {
            x_mult = -1;
        }
        if diff.y < 0 {
            y_mult = -1;
        }

        if head.x == tail.x {
            return Position { x: 0, y: y_mult };
        } else if head.y == tail.y {
            return Position { x: x_mult, y: 0 };
        } else {
            return Position {
                x: x_mult,
                y: y_mult,
            };
        }
    }

    fn make_move(&mut self, mv: Move) {
        let qty = mv.move_quantity();
        for _ in 0..qty {
            self.positions[0] += mv.to_position_offset();

            for i in 1..self.positions.len() {
                let pos_mv = Bridge::calc_move_tail(&self.positions[i - 1], &self.positions[i]);
                self.positions[i] += pos_mv;
            }

            self.visited
                .insert(self.positions[self.positions.len() - 1]);
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Add for Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Position;
    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl Position {
    fn adjacent(&self, other: &Self) -> bool {
        let neighbours = [
            *self + Position { x: -1, y: 0 },
            *self + Position { x: 1, y: 0 },
            *self + Position { x: 0, y: 1 },
            *self + Position { x: 0, y: -1 },
            *self + Position { x: 1, y: 1 },
            *self + Position { x: 1, y: -1 },
            *self + Position { x: -1, y: 1 },
            *self + Position { x: -1, y: -1 },
        ];
        neighbours.iter().any(|x| *x == *other)
    }
}
fn main() {
    let mut bridge = Bridge::new(10);
    let content = fs::read_to_string("input.txt").expect("the file should be on the disk");
    content
        .lines()
        .map(|l| Move::try_from(l.to_string()).unwrap())
        .for_each(|mv| {
            bridge.make_move(mv);
        });
    // println!("{:?}", bridge);
    println!("{}", bridge.visited.len());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_move_parse() {
        assert_eq!(Move::try_from("R 10".to_string()), Ok(Move::Right(10)));
        assert_eq!(Move::try_from("L 10".to_string()), Ok(Move::Left(10)));
        assert_eq!(Move::try_from("U 10".to_string()), Ok(Move::Up(10)));
        assert_eq!(Move::try_from("D 10".to_string()), Ok(Move::Down(10)));
        assert_eq!(Move::try_from("R 100".to_string()), Ok(Move::Right(100)));
        assert_eq!(Move::try_from("L 100".to_string()), Ok(Move::Left(100)));
    }

    #[test]
    fn test_add_positions() {
        assert_eq!(
            Position { x: 1, y: 1 } + Position { x: -2, y: -3 },
            Position { x: -1, y: -2 }
        );
        let mut p = Position { x: 1, y: 1 };
        p += Position { x: -2, y: -3 };
        assert_eq!(p, Position { x: -1, y: -2 });
    }

    #[test]
    fn sub_postions() {
        assert_eq!(
            Position { x: 1, y: 1 } - Position { x: -2, y: -3 },
            Position { x: 3, y: 4 }
        )
    }

    #[test]
    fn neighbours() {
        let p = Position { x: 4, y: 4 };
        assert!(p.adjacent(&Position { x: 4, y: 3 }));
        assert!(p.adjacent(&Position { x: 3, y: 3 }));
        assert!(p.adjacent(&Position { x: 5, y: 5 }));
        assert!(!p.adjacent(&Position { x: 2, y: 3 }));
    }
}
