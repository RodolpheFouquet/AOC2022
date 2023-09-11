use std::cmp;
use std::fs;

struct Forest {
    trees: Vec<Vec<u32>>,
}

impl TryFrom<String> for Forest {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Forest {
            trees: value
                .lines()
                .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .collect(),
        })
    }
}

pub trait CountWhileExt: Iterator {
    fn count_while<P: Fn(Self::Item) -> bool>(self, predicate: P) -> usize;
}

// apply trait to all types implementing the Iterator trait.
impl<I> CountWhileExt for I
where
    I: Iterator,
    I::Item: Copy + Clone + Ord,
{
    fn count_while<P: Fn(Self::Item) -> bool>(self, predicate: P) -> usize {
        let mut count_ended = false;
        self.fold(0, |acc, x| {
            if predicate(x) && !count_ended {
                acc + 1
            } else if !count_ended {
                count_ended = true;
                acc + 1
            } else {
                acc
            }
        })
    }
}

impl Forest {
    fn visible(&self) -> usize {
        let mut count = self.trees.len() * 2 + self.trees[0].len() * 2 - 4;
        for i in 1..self.trees.len() - 1 {
            for j in 1..self.trees[i].len() - 1 {
                let left = !self.trees[i][0..j]
                    .to_vec()
                    .iter()
                    .any(|&x| x >= self.trees[i][j]);
                let right = !self.trees[i][j + 1..self.trees[i].len()]
                    .to_vec()
                    .iter()
                    .any(|&x| x >= self.trees[i][j]);

                let top = !self.trees[0..i]
                    .iter()
                    .map(|v| v[j])
                    .any(|x| x >= self.trees[i][j]);

                let bottom = !self.trees[i + 1..self.trees.len()]
                    .iter()
                    .map(|v| v[j])
                    .any(|x| x >= self.trees[i][j]);

                if left || right || top || bottom {
                    count += 1;
                }
            }
        }

        count
    }

    fn scenic_score(&self) -> usize {
        let mut max = 0;
        for i in 0..self.trees.len() {
            for j in 0..self.trees[i].len() {
                let here = self.trees[i][j];
                let left_score = self.trees[i][0..j]
                    .to_vec()
                    .into_iter()
                    .rev()
                    .count_while(|x| x < self.trees[i][j]);
                let right_score = self.trees[i][j + 1..self.trees[i].len()]
                    .to_vec()
                    .into_iter()
                    .count_while(|x| x < self.trees[i][j]);

                let top_score = self.trees[0..i]
                    .iter()
                    .rev()
                    .map(|v| v[j])
                    .count_while(|x| x < self.trees[i][j]);

                let bottom_score = self.trees[i + 1..self.trees.len()]
                    .iter()
                    .map(|v| v[j])
                    .count_while(|x| x < self.trees[i][j]);
                max = cmp::max(max, left_score * right_score * top_score * bottom_score);
            }
        }
        max
    }
}

fn main() {
    let content = fs::read_to_string("input2.txt").expect("the input should be on the disk");
    let forest = Forest::try_from(content).unwrap();
    println!(
        "The number of trees that can be seen is {}",
        forest.visible()
    );
    println!("The max score is {}", forest.scenic_score());
}

#[cfg(test)]
mod tests {

    use crate::{CountWhileExt, Forest};

    #[test]
    fn test_forest_parse() {
        let forest = Forest::try_from("213\n321".to_string()).unwrap();
        assert_eq!(forest.trees, vec![vec![2, 1, 3], vec![3, 2, 1]]);
    }

    #[test]
    fn test_count_visible() {
        let trees = "30373
25512
65332
33549
35390";
        let forest = Forest::try_from(trees.to_string()).unwrap();
        assert_eq!(forest.visible(), 21);
    }

    #[test]
    fn test_count_until() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(v.into_iter().count_while(|x| x <= 3), 4);
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(v.into_iter().count_while(|x| x < 1), 1);
    }

    #[test]
    fn test_score() {
        let trees = "30373
25512
65332
33549
35390";
        let forest = Forest::try_from(trees.to_string()).unwrap();
        assert_eq!(forest.scenic_score(), 8);
    }
}
