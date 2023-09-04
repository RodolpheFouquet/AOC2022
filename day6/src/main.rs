use std::{collections::HashSet, fs};

fn find_start_packet(input: &str, consecutive: usize) -> usize {
    let mut ret = 0;
    let mut found = false;
    let mut i = consecutive - 1;
    while i < input.len() && !found {
        let mut set: HashSet<char> = HashSet::new();
        ret = i;
        for j in 0..consecutive {
            let c = input.chars().nth(i - j).unwrap();
            if set.contains(&c) {
                break;
            }
            set.insert(c);
            if set.len() == consecutive {
                found = true;
            }
        }
        i += 1;
    }
    ret + 1
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let ret = find_start_packet(content.as_str(), 4);
    println!("{}", ret);
    let ret = find_start_packet(content.as_str(), 14);
    println!("{}", ret);
}

#[cfg(test)]
mod tests {
    use crate::find_start_packet;

    #[test]
    fn test_find_start_packet() {
        let inputs = vec![
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 4, 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 4, 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4, 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4, 11),
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14, 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 14, 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 14, 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14, 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14, 26),
        ];

        inputs
            .iter()
            .for_each(|test| assert_eq!(find_start_packet(test.0, test.1), test.2))
    }
}
