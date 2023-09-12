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
    println!("{}", strength);
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
