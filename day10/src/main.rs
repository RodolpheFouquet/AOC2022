use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum Command {
    Noop,
    Addx(i32),
}

impl TryFrom<&str> for Command {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split_whitespace();
        let first = split.next().unwrap();
        match first {
            "noop" => Ok(Command::Noop),
            "addx" => Ok(Command::Addx(
                split
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .map_err(|_| ())
                    .unwrap(),
            )),
            _ => Err(()),
        }
    }
}

struct Screen {
    sprite: i32,
    pixels: [[char; 40]; 6],
}

impl Screen {
    fn new() -> Self {
        Self {
            sprite: 0,
            pixels: [[' '; 40]; 6],
        }
    }

    fn move_sprite(&mut self, pos: i32) {
        self.sprite = pos;
    }

    fn visible(&self) -> (i32, i32, i32) {
        (self.sprite - 1, self.sprite, self.sprite + 1)
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pixels.iter().for_each(|line| {
            line.iter().for_each(|c| {
                write!(f, "{}", c);
            });
            writeln!(f, "");
        });
        write!(f, "")
    }
}

fn main() {
    let mut cycles: Vec<i32> = vec![];
    let cycle_num: [i32; 6] = [20, 60, 100, 140, 180, 220];
    let mut current_value = 1;
    let commands: Vec<Command> = std::fs::read_to_string("input2.txt")
        .unwrap()
        .lines()
        .map(Command::try_from)
        .map(|c| c.unwrap())
        .collect();

    for c in commands {
        match c {
            Command::Noop => {
                cycles.push(current_value);
            }
            Command::Addx(x) => {
                cycles.push(current_value);
                cycles.push(current_value);
                current_value += x;
            }
        }
    }

    let strength: i32 = cycle_num
        .to_vec()
        .iter()
        .map(|&x| cycles[(x - 1) as usize] * x)
        .sum();

    let mut screen = Screen::new();
    println!("{}", strength);
    println!("{}", cycles.len());

    for i in 0..6 {
        for j in 0..40 {
            screen.move_sprite(cycles[i * 40 + j]);
            let (x1, x2, x3) = screen.visible();

            if j as i32 == x1 || j as i32 == x2 || j as i32 == x3 {
                screen.pixels[i][j] = '#';
            } else {
                screen.pixels[i][j] = '.';
            }
        }
    }
    println!("{}", screen);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        assert_eq!(Command::try_from("noop"), Ok(Command::Noop));
        assert_eq!(Command::try_from("addx 1"), Ok(Command::Addx(1)));
    }
}
