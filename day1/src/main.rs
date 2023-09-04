use std::fs;

// result is 66186
fn main() {
    let mut maxs = (0, 0, 0);
    let content =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");

    content.lines().fold(0, |acc, line| {
        let res = line.parse::<i32>();
        match res {
            Ok(v) => acc + v,
            Err(_) => {
                if maxs.0 < acc {
                    maxs = (acc, maxs.0, maxs.1);
                } else if maxs.1 < acc {
                    maxs = (maxs.0, acc, maxs.1);
                } else if maxs.2 < acc {
                    maxs = (maxs.0, maxs.1, acc);
                }
                0
            }
        }
    });

    println!("{} {} {}", maxs.0, maxs.1, maxs.2);
    println!("{}", maxs.0 + maxs.1 + maxs.2);
}
