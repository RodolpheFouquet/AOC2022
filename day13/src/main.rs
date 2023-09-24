use serde::Deserialize;
use serde_json::Number;
use std::cmp::Ordering;

#[derive(Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
enum Element {
    Integer(u32),
    List(Vec<Element>),
}

impl Element {
    fn with_slice<T>(&self, f: impl FnOnce(&[Element]) -> T) -> T {
        match self {
            Self::List(n) => f(&n[..]),
            Self::Integer(n) => f(&[Self::Integer(*n)]),
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Element::Integer(a), Element::Integer(b)) => a.partial_cmp(&b),
            (l, r) => Some(l.with_slice(|l| {
                r.with_slice(|r| {
                    l.iter()
                        .zip(r.iter())
                        .map(|(a, b)| a.cmp(b))
                        // return the first ordering that isn't `Equal`
                        .find(|&ord| ord != Ordering::Equal)
                        // or compare the lengths
                        .unwrap_or_else(|| l.len().cmp(&r.len()))
                })
            })),
        }
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::List(list) => f.debug_list().entries(list).finish(),
        }
    }
}

fn main() {
    let groups = include_str!("../input2.txt").split("\n\n");
    let packet_2: Element = serde_json::from_str("[[2]]").unwrap();
    let packet_6: Element = serde_json::from_str("[[6]]").unwrap();
    let mut packets: Vec<Element> = groups
        .clone()
        .flat_map(|lines| {
            lines
                .lines()
                .map(|l| serde_json::from_str::<Element>(l).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    packets.push(packet_2.clone());
    packets.push(packet_6.clone());
    packets.sort();

    let position_2 = packets.iter().position(|p| *p == packet_2).unwrap() + 1;
    let position_6 = packets.iter().position(|p| *p == packet_6).unwrap() + 1;
    let mut sum = 0;
    for (i, group) in groups.enumerate() {
        let mut elements = group
            .lines()
            .map(|line| serde_json::from_str::<Element>(line).unwrap());
        let left = elements.next().unwrap();
        let right = elements.next().unwrap();

        if left < right {
            sum += i + 1;
        }
    }
    println!("{:?}", sum);
    println!("{:?}", position_6 * position_2);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_compare() {
        let l1 = Element::Integer(10);
        let l2 = Element::Integer(2);
        let l3 = Element::Integer(2);

        assert_eq!(l1.partial_cmp(&l2), Some(Ordering::Greater));
        assert_eq!(l2.partial_cmp(&l1), Some(Ordering::Less));
        assert_eq!(l2.partial_cmp(&l3), Some(Ordering::Equal));

        let l1 = Element::List(vec![]);
        let l2 = Element::List(vec![Element::Integer(1)]);
        assert_eq!(l1.partial_cmp(&l2), Some(Ordering::Less));
        assert_eq!(l2.partial_cmp(&l1), Some(Ordering::Greater));

        let l1 = Element::Integer(1);
        let l2 = Element::List(vec![Element::Integer(2)]);
        assert_eq!(l1.partial_cmp(&l2), Some(Ordering::Less));
        assert_eq!(l2.partial_cmp(&l1), Some(Ordering::Greater));

        let l1 = Element::List(vec![Element::Integer(1)]);
        let l2 = Element::List(vec![Element::Integer(2)]);
        assert_eq!(l1.partial_cmp(&l2), Some(Ordering::Less));
        assert_eq!(l2.partial_cmp(&l1), Some(Ordering::Greater));
    }
}
