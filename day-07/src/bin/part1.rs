#![allow(unused)]
use std::fmt;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut all_hands: Vec<Hand> = input
        .lines()
        .map(|line| Hand::new(line.to_string()))
        .collect();

    // test a basic sort
    all_hands.sort_by(|a, b| a.bid.cmp(&b.bid));

    println!("{:#?}", all_hands);
    "todo!()".to_string()
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    cards: String,
    bid: u32,
}

impl Hand {
    pub fn new(line: String) -> Self {
        let split_line = line.split_once(" ").unwrap();
        Self {
            cards: split_line.0.to_string(),
            bid: split_line.1.parse::<u32>().unwrap(),
        }
    }
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hand {{ Cards: {}, Bid: {} }}", self.cards, self.bid)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, "6440".to_string());
    }
}
