use std::error;
use std::fs;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Range {
    low: i32,
    high: i32,
}

impl Range {
    fn contains(&self, r: &Range) -> bool {
        return self.low <= r.low && self.high >= r.high;
    }
    fn overlaps(&self, r: &Range) -> bool {
        (r.high - self.low) * (self.high - r.low) >= 0
    }
}

#[derive(PartialEq, Debug)]
struct MissingBoundaryError(String);

impl TryFrom<String> for Range {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut comps = s.split('-');
        let low = comps.next();
        let high = comps.next();

        match (low, high) {
            (Some(l), Some(h)) => Ok(Range {
                low: l.parse::<i32>().map_err(|_| ())?,
                high: h.parse::<i32>().map_err(|_| ())?,
            }),
            (_, _) => Err(()),
        }
    }
}

fn parse_line(line: &str) -> Result<(Range, Range), ()> {
    let mut comps = line.split(",");
    let first = comps.next();
    let second = comps.next();

    match (first, second) {
        (Some(f), Some(s)) => Ok((
            Range::try_from(f.to_string())?,
            Range::try_from(s.to_string())?,
        )),
        (_, _) => Err(()),
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("this file should exist");
    let ret = content
        .lines()
        .map(parse_line)
        .filter(|pair| match pair {
            Ok(p) => p.0.contains(&p.1) || p.1.contains(&p.0),
            Err(_) => unreachable!(),
        })
        .count();
    println!("{}", ret);
    let ret = content
        .lines()
        .map(parse_line)
        .filter(|pair| match pair {
            Ok(p) => p.0.overlaps(&p.1),
            Err(_) => unreachable!(),
        })
        .count();
    println!("{}", ret);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_try_from() {
        let r = Range::try_from(String::from("1-100"));
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), Range { low: 1, high: 100 });

        let r = Range::try_from(String::from("1-"));
        assert!(r.is_err());

        let r = Range::try_from(String::from("1-99.9"));
        assert!(r.is_err());

        let r = Range::try_from(String::from("1-asdasd"));
        assert!(r.is_err());
    }

    #[test]
    fn tets_parse_line() {
        let r = parse_line("1-2,3-4");
        assert!(r.is_ok());
        assert_eq!(
            r.unwrap(),
            (Range { low: 1, high: 2 }, Range { low: 3, high: 4 })
        );
        let r = parse_line("1-2");
        assert!(r.is_err());
        let r = parse_line("1-2,");
        assert!(r.is_err());
        let r = parse_line("1-2,3-99.99");
        assert!(r.is_err());
        let r = parse_line("1-asjdbna,3-99");
        assert!(r.is_err());
    }

    #[test]
    fn test_contains() {
        assert!(Range { low: 50, high: 100 }.contains(&Range { low: 55, high: 95 }));
        assert!(Range { low: 50, high: 100 }.contains(&Range { low: 99, high: 99 }));
        assert!(Range { low: 4, high: 6 }.contains(&Range { low: 6, high: 6 }));

        assert!(!Range { low: 50, high: 100 }.contains(&Range { low: 55, high: 105 }));
        assert!(!Range { low: 50, high: 100 }.contains(&Range { low: 45, high: 105 }));
        assert!(!Range { low: 50, high: 100 }.contains(&Range { low: 45, high: 95 }));
    }

    #[test]
    fn test_overlaps() {
        assert!(Range { low: 5, high: 7 }.overlaps(&Range { low: 7, high: 9 }));
        assert!(Range { low: 2, high: 8 }.overlaps(&Range { low: 3, high: 7 }));
        assert!(Range { low: 6, high: 6 }.overlaps(&Range { low: 4, high: 6 }));
        assert!(Range { low: 2, high: 6 }.overlaps(&Range { low: 4, high: 8 }));
        assert!(!Range { low: 2, high: 3 }.overlaps(&Range { low: 4, high: 8 }));
        assert!(!Range { low: 4, high: 8 }.overlaps(&Range { low: 2, high: 3 }));
    }
}
