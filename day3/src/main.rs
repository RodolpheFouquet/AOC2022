use std::{collections::HashSet, fs};

fn split_inputs(input: &str) -> (&str, &str) {
    input.split_at(input.len() / 2)
}

fn get_priority(ch: &char) -> u32 {
    if !ch.is_ascii_alphabetic() {
        unreachable!()
    }
    if ch.is_ascii_lowercase() {
        *ch as u32 - 96
    } else if ch.is_ascii_uppercase() {
        *ch as u32 - 64 + 26
    } else {
        unreachable!()
    }
}

fn get_dupes(input: (&str, &str)) -> HashSet<char> {
    let mut ret = HashSet::new();
    for i in 0..input.0.len() {
        for j in 0..input.1.len() {
            let (a, b) = (input.0.chars().nth(i), input.1.chars().nth(j));
            match (a, b) {
                (Some(c1), Some(c2)) => {
                    if c1 == c2 {
                        ret.insert(c1);
                    }
                }
                _ => break,
            }
        }
    }
    ret
}

fn get_prio_sum(v: HashSet<char>) -> u32 {
    v.iter().map(get_priority).sum()
}

fn main() {
    let content =
        fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let res: u32 = content
        .lines()
        .map(split_inputs)
        .map(get_dupes)
        .map(get_prio_sum)
        .sum();
    println!("{}", res);

    let lines: Vec<HashSet<char>> = content
        .lines()
        .map(|line| {
            let ret: HashSet<char> = line.chars().collect();
            ret
        })
        .collect();
    let sum: u32 = lines
        .chunks(3)
        .map(|group| {
            let inter: HashSet<char> = group[0].intersection(&group[1]).copied().collect();
            let inter2: HashSet<char> = group[2].intersection(&inter).copied().collect();
            inter2
        })
        .map(get_prio_sum)
        .sum();
    println!("{}", sum)
}

#[cfg(test)]
mod tests {
    use crate::{get_dupes, get_prio_sum, get_priority, split_inputs};

    #[test]
    fn test_split_inputs() {
        let (a, b) = split_inputs("rust");
        assert_eq!(a, "ru");
        assert_eq!(b, "st");
    }

    #[test]
    fn test_get_dupes() {
        let dupes = get_dupes(split_inputs("dictpixc"));
        assert_eq!(2, dupes.len());
        assert!(dupes.contains(&'c'));
        assert!(dupes.contains(&'i'));
    }

    #[test]
    fn test_get_prio_sum() {
        let dupes = get_dupes(split_inputs("dictpixc"));
        let res = get_prio_sum(dupes);
        assert_eq!(get_priority(&'c') + get_priority(&'i'), res);
    }

    #[test]
    fn test_iget_prio() {
        assert_eq!(1, get_priority(&'a'));
        assert_eq!(2, get_priority(&'b'));
        assert_eq!(26, get_priority(&'z'));
        assert_eq!(27, get_priority(&'A'));
        assert_eq!(52, get_priority(&'Z'));
    }
}
