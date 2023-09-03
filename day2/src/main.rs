use std::fs;

#[derive(PartialEq, Debug, Clone)]
enum RockPaperScissors {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq, Debug)]
enum WinLoseDraw {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

fn convert_to_rps(val: &str) -> Result<RockPaperScissors, String> {
    if val == "A" || val == "X" {
        Ok(RockPaperScissors::Rock)
    } else if val == "B" || val == "Y" {
        Ok(RockPaperScissors::Paper)
    } else if val == "C" || val == "Z" {
        Ok(RockPaperScissors::Scissors)
    } else {
        Err(format!(
            "unexpected {}, expected RockPaperScissors or XYZ",
            val
        ))
    }
}

fn convert_to_wld(val: &str) -> Result<WinLoseDraw, String> {
    match val {
        "X" => Ok(WinLoseDraw::Lose),
        "Y" => Ok(WinLoseDraw::Draw),
        "Z" => Ok(WinLoseDraw::Win),
        _ => Err(format!("Unexpected input {}, expected X Y Z", val)),
    }
}

fn line_to_play(line: &str) -> Result<(RockPaperScissors, RockPaperScissors), String> {
    let mut content = line.split_whitespace();

    let their_move = content.next();
    let my_move = content.next();

    match (their_move, my_move) {
        (Some(theirs), Some(mine)) => Ok((convert_to_rps(theirs)?, convert_to_rps(mine)?)),
        (_, _) => Err(format!("unexpected line format {}", line)),
    }
}

fn counterplay_to_play(
    counter: (RockPaperScissors, WinLoseDraw),
) -> (RockPaperScissors, RockPaperScissors) {
    match counter {
        (play, WinLoseDraw::Draw) => (play.clone(), play),
        (play, WinLoseDraw::Win) => (play.clone(), win_move(&play)),
        (play, WinLoseDraw::Lose) => (play.clone(), lose_move(&play)),
    }
}

fn line_to_counterplay(line: &str) -> Result<(RockPaperScissors, RockPaperScissors), String> {
    let mut content = line.split_whitespace();

    let their_move = content.next();
    let my_move = content.next();

    match (their_move, my_move) {
        (Some(theirs), Some(mine)) => Ok(counterplay_to_play((
            convert_to_rps(theirs)?,
            convert_to_wld(mine)?,
        ))),
        (_, _) => Err(format!("unexpected line format {}", line)),
    }
}

fn win_move(mv: &RockPaperScissors) -> RockPaperScissors {
    match mv {
        RockPaperScissors::Rock => RockPaperScissors::Paper,
        RockPaperScissors::Paper => RockPaperScissors::Scissors,
        RockPaperScissors::Scissors => RockPaperScissors::Rock,
    }
}
fn lose_move(mv: &RockPaperScissors) -> RockPaperScissors {
    match mv {
        RockPaperScissors::Scissors => RockPaperScissors::Paper,
        RockPaperScissors::Rock => RockPaperScissors::Scissors,
        RockPaperScissors::Paper => RockPaperScissors::Rock,
    }
}

fn is_win(play: (&RockPaperScissors, &RockPaperScissors)) -> bool {
    match play {
        (RockPaperScissors::Rock, RockPaperScissors::Paper) => true,
        (RockPaperScissors::Paper, RockPaperScissors::Scissors) => true,
        (RockPaperScissors::Scissors, RockPaperScissors::Rock) => true,
        (_, _) => false,
    }
}

fn calculate_outcome(play: (RockPaperScissors, RockPaperScissors)) -> i32 {
    match play {
        (v1, v2) if v1 == v2 => 3 + v2 as i32,
        (v1, v2) if is_win((&v1, &v2)) => 6 + v2 as i32,
        (_, v2) => v2 as i32,
    }
}

fn main() {
    let content =
        fs::read_to_string("input.txt").expect("LogRocket: Should have been able to read the file");

    let result = content
        .lines()
        .map(line_to_counterplay)
        .fold(0, |acc, pair| match pair {
            Ok(p) => acc + calculate_outcome(p),
            Err(e) => {
                println!("{}", e);
                unreachable!()
            }
        });

    println!("the result is {}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_rps() {
        assert_eq!(convert_to_rps("A").unwrap(), RockPaperScissors::Rock);
        assert_eq!(convert_to_rps("X").unwrap(), RockPaperScissors::Rock);
        assert_eq!(convert_to_rps("B").unwrap(), RockPaperScissors::Paper);
        assert_eq!(convert_to_rps("Y").unwrap(), RockPaperScissors::Paper);
        assert_eq!(convert_to_rps("C").unwrap(), RockPaperScissors::Scissors);
        assert_eq!(convert_to_rps("Z").unwrap(), RockPaperScissors::Scissors);
        assert!(convert_to_rps("k").is_err());
    }

    #[test]
    fn test_convert_to_wld() {
        assert_eq!(convert_to_wld("X").unwrap(), WinLoseDraw::Lose);
        assert_eq!(convert_to_wld("Y").unwrap(), WinLoseDraw::Draw);
        assert_eq!(convert_to_wld("Z").unwrap(), WinLoseDraw::Win);
    }

    #[test]
    fn test_outcome() {
        // Draws
        assert_eq!(
            calculate_outcome((RockPaperScissors::Scissors, RockPaperScissors::Scissors)),
            6
        );
        assert_eq!(
            calculate_outcome((RockPaperScissors::Rock, RockPaperScissors::Rock)),
            4
        );
        assert_eq!(
            calculate_outcome((RockPaperScissors::Paper, RockPaperScissors::Paper)),
            5
        );

        // Wins
        assert_eq!(
            calculate_outcome((RockPaperScissors::Paper, RockPaperScissors::Scissors)),
            9
        );
        assert_eq!(
            calculate_outcome((RockPaperScissors::Rock, RockPaperScissors::Paper)),
            8
        );
        assert_eq!(
            calculate_outcome((RockPaperScissors::Scissors, RockPaperScissors::Rock)),
            7
        );

        // Losses
        assert_eq!(
            calculate_outcome((RockPaperScissors::Paper, RockPaperScissors::Rock)),
            1
        );
        assert_eq!(
            calculate_outcome((RockPaperScissors::Rock, RockPaperScissors::Scissors)),
            3
        );
        assert_eq!(
            calculate_outcome((RockPaperScissors::Scissors, RockPaperScissors::Paper)),
            2
        );
    }

    #[test]
    fn test_line_to_play() {
        assert_eq!(
            line_to_play("A X").unwrap(),
            (RockPaperScissors::Rock, RockPaperScissors::Rock)
        );
        assert_eq!(
            line_to_play("B X").unwrap(),
            (RockPaperScissors::Paper, RockPaperScissors::Rock)
        );
        assert_eq!(
            line_to_play("C X").unwrap(),
            (RockPaperScissors::Scissors, RockPaperScissors::Rock)
        );
        assert_eq!(
            line_to_play("C Y").unwrap(),
            (RockPaperScissors::Scissors, RockPaperScissors::Paper)
        );
        assert_eq!(
            line_to_play("C Z").unwrap(),
            (RockPaperScissors::Scissors, RockPaperScissors::Scissors)
        );
        assert!(line_to_play("x Z").is_err());
        assert!(line_to_play("A").is_err());
        assert!(line_to_play("A x").is_err());
    }
}
