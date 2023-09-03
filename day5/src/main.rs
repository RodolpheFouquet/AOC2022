use std::fmt;
use std::fs;

#[derive(Debug)]
struct Stack {
    v: Vec<String>,
}

impl Stack {
    fn new() -> Self {
        Stack { v: Vec::new() }
    }

    fn push(&mut self, s: String) {
        self.v.push(s);
    }

    fn pop(&mut self) -> Option<String> {
        self.v.pop()
    }

    fn top(&self) -> Option<String> {
        self.v.last().cloned()
    }
}

struct Move {
    quantity: u32,
    source: usize,
    destination: usize,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "move {} from {} to {}",
            self.quantity, self.source, self.destination
        )
    }
}

impl TryFrom<String> for Move {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let comps: Vec<&str> = s.split(' ').collect();
        Ok(Self {
            quantity: comps.get(1).unwrap().parse::<u32>().map_err(|_| ())?,
            source: comps.get(3).unwrap().parse::<usize>().map_err(|_| ())?,
            destination: comps.get(5).unwrap().parse::<usize>().map_err(|_| ())?,
        })
    }
}

#[derive(Debug)]
struct Crane {
    stacks: Vec<Stack>,
}

impl Crane {
    fn new() -> Self {
        Crane { stacks: Vec::new() }
    }

    fn move_boxes(&mut self, mv: &Move) {
        (0..mv.quantity).for_each(|_| {
            let e = self.stacks.get_mut(mv.source - 1).unwrap().pop().unwrap();
            self.stacks.get_mut(mv.destination - 1).unwrap().push(e);
        })
    }

    fn move_boxes_in_order(&mut self, mv: &Move) {
        let boxes: Vec<String> = (0..mv.quantity)
            .map(|_| self.stacks.get_mut(mv.source - 1).unwrap().pop().unwrap())
            .collect();
        boxes
            .into_iter()
            .rev()
            .for_each(|b| self.stacks.get_mut(mv.destination - 1).unwrap().push(b));
    }
}

impl fmt::Display for Crane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let highest_stack = self.stacks.iter().map(|s| s.v.len()).max().unwrap();
        let mut output = String::new();
        for i in 0..self.stacks.len() {
            output.push_str(format!(" {} ", i + 1).as_str());
            if i < self.stacks.len() {
                output.push(' ');
            }
        }
        output.push('\n');

        for i in 0..highest_stack {
            for y in 0..self.stacks.len() {
                match self.stacks.get(y).unwrap().v.get(i) {
                    Some(e) => output.push_str(format!("[{}]", e).as_str()),
                    None => output.push_str("   "),
                };
                if y < self.stacks.len() {
                    output.push(' ');
                }
            }
            output.push('\n');
        }
        output.lines().rev().for_each(|l| {
            writeln!(f, "{}", l).unwrap();
        });
        writeln!(f)
    }
}

impl TryFrom<String> for Crane {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut rev_lines = s.lines().rev();
        let numbers = rev_lines.next();
        let num_of_cols = match numbers {
            Some(n) => Ok(n.split_whitespace().count()),
            None => Err(()),
        };

        let num = num_of_cols?;
        let mut ret = Crane::new();

        ret.stacks.resize_with(num, Stack::new);
        rev_lines.for_each(|l| {
            let mut counter = 0;
            let mut col = 0;
            l.chars().for_each(|c| match c {
                '[' => {
                    counter = 0;
                }
                ']' => {}
                ' ' => {
                    counter += 1;
                    counter %= 4;
                    if counter == 3 {
                        col += 1;
                    }
                }
                c => {
                    ret.stacks[col].push(c.to_string());
                    col += 1;
                }
            });
        });

        Ok(ret)
    }
}

fn main() {
    let file = fs::read_to_string("input1.txt").expect("the file must be present on the disk");

    let mut crane_lines: Vec<String> = Vec::new();
    let mut move_lines: Vec<String> = Vec::new();

    let mut crane_filled = false;
    file.lines().for_each(|line| {
        if line.is_empty() {
            crane_filled = true;
            return;
        }

        if crane_filled {
            move_lines.push(line.to_string());
        } else {
            crane_lines.push(line.to_string());
        }
    });

    let crane_string = crane_lines.join("\n");
    let mut c = Crane::try_from(crane_string.to_string()).unwrap();
    print!("{}", c);

    move_lines
        .iter()
        .map(|mv_string| Move::try_from(mv_string.to_string()).unwrap())
        .for_each(|mv| c.move_boxes_in_order(&mv));
    print!("{}", c);
    c.stacks.iter().for_each(|s| print!("{}", s.top().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = Stack::new();
        assert_eq!(s.top(), None);
    }

    #[test]
    fn test_stack_manipulation() {
        let mut s = Stack::new();
        assert_eq!(s.top(), None);

        s.push(String::from("A"));
        assert_eq!(s.top(), Some(String::from("A")));

        s.push(String::from("B"));
        assert_eq!(s.top(), Some(String::from("B")));

        assert_eq!(s.pop(), Some(String::from("B")));
        assert_eq!(s.pop(), Some(String::from("A")));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_crane_init() {
        let s = String::from(
            "    [P]                 [Q]     [T]
[F] [N]             [P] [L]     [M]
[H] [T] [H]         [M] [H]     [Z]
[M] [C] [P]     [Q] [R] [C]     [J]
[T] [J] [M] [F] [L] [G] [R]     [Q]
[V] [G] [D] [V] [G] [D] [N] [W] [L]
[L] [Q] [S] [B] [H] [B] [M] [L] [D]
[D] [H] [R] [L] [N] [W] [G] [C] [R]
 1   2   3   4   5   6   7   8   9 ",
        );
        let crane = Crane::try_from(s);
        assert!(cra.rev()ne.is_ok());
        assert_eq!(crane.unwrap().stacks.len(), 9);
    }

    #[test]
    #[should_panic]
    fn test_crate_init_panic() {
        let s2 = String::from(
            "                    [Q]     [T]
[F] [N]             [P] [L]     [M]
[H] [T] [H]         [M] [H]     [Z]
[M] [C] [P]     [Q] [R] [C]     [J]
[T] [J] [M] [F] [L] [G] [R]     [Q]
[V] [G] [D] [V] [G] [D] [N] [W] [L]
[L] [Q] [S] [B] [H] [B] [M] [L] [D]
[D] [H] [R] [L] [N] [W] [G] [C] [R]
 1   2   3   4   5   7   8   9 ",
        );
        let crane2 = Crane::try_from(s2);
        assert!(crane2.is_ok());
    }

    #[test]
    fn test_crane_move() {
        let s = String::from(
            "    [P]                 [Q]     [T]
[F] [N]             [P] [L]     [M]
[H] [T] [H]         [M] [H]     [Z]
[M] [C] [P]     [Q] [R] [C]     [J]
[T] [J] [M] [F] [L] [G] [R]     [Q]
[V] [G] [D] [V] [G] [D] [N] [W] [L]
[L] [Q] [S] [B] [H] [B] [M] [L] [D]
[D] [H] [R] [L] [N] [W] [G] [C] [R]
 1   2   3   4   5   6   7   8   9 ",
        );

        let mut crane = Crane::try_from(s).unwrap();
        assert_eq!(crane.stacks.len(), 9);
        assert_eq!(
            crane.stacks.last().unwrap().top().unwrap(),
            String::from("T")
        );
        assert_eq!(
            crane.stacks.get(3).unwrap().top().unwrap(),
            String::from("F")
        );
        crane.move_boxes(&Move {
            quantity: 2,
            source: 9,
            destination: 4,
        });

        assert_eq!(
            crane.stacks.last().unwrap().top().unwrap(),
            String::from("Z")
        );
        assert_eq!(
            crane.stacks.get(3).unwrap().top().unwrap(),
            String::from("M")
        );
    }

    #[test]
    fn test_crane_move_in_order() {
        let s = String::from(
            "    [P]                 [Q]     [T]
[F] [N]             [P] [L]     [M]
[H] [T] [H]         [M] [H]     [Z]
[M] [C] [P]     [Q] [R] [C]     [J]
[T] [J] [M] [F] [L] [G] [R]     [Q]
[V] [G] [D] [V] [G] [D] [N] [W] [L]
[L] [Q] [S] [B] [H] [B] [M] [L] [D]
[D] [H] [R] [L] [N] [W] [G] [C] [R]
 1   2   3   4   5   6   7   8   9 ",
        );

        let mut crane = Crane::try_from(s).unwrap();
        assert_eq!(crane.stacks.len(), 9);
        assert_eq!(
            crane.stacks.last().unwrap().top().unwrap(),
            String::from("T")
        );
        assert_eq!(
            crane.stacks.get(3).unwrap().top().unwrap(),
            String::from("F")
        );
        crane.move_boxes_in_order(&Move {
            quantity: 2,
            source: 9,
            destination: 4,
        });

        assert_eq!(
            crane.stacks.last().unwrap().top().unwrap(),
            String::from("Z")
        );
        assert_eq!(
            crane.stacks.get(3).unwrap().top().unwrap(),
            String::from("T")
        );
    }

    #[test]
    fn parse_move() {
        let input = String::from("move 1 from 2 to 3");
        let mv = Move::try_from(input);

        assert!(mv.is_ok());
        let mv = mv.unwrap();
        assert_eq!(mv.quantity, 1);
        assert_eq!(mv.source, 2);
        assert_eq!(mv.destination, 3);
    }
}
