use std::{collections::{VecDeque, HashSet}, fs};

fn find_start_packet(input: &str) -> usize {
   
    let mut ret = 0;
    let mut found = false;
    let mut i = 3;
    while i < input.len() && !found {
        let mut set:  HashSet<char> = HashSet::new();
        ret = i;
        for j in 0..4 {
            let c = input.chars().into_iter().nth(i-j).unwrap();
            if set.contains(&c) {
                break;
            }
            set.insert(c);
            if set.len() == 4{
                found = true;
            }
        }
        i +=1;
    }
    ret+1
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let ret = find_start_packet(content.as_str());
    println!("{}", ret);
}

#[cfg(test)]
mod tests {
    use crate::find_start_packet;

    #[test]
    fn test_find_start_packet() {
        let inputs = vec![
             ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        inputs.iter().for_each(|test| {
            assert_eq!(find_start_packet(test.0), test.1)
        })
    }
}
