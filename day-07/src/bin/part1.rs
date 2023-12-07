#![allow(unused)]
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
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
    all_hands.sort_by(|a, b| b.cmp(a));
    //    all_hands.sort_by(|a, b| a.bid.cmp(&b.bid));

    println!("{:#?}", all_hands);
    "todo!()".to_string()
}

#[derive(Debug)]
enum HandType {
    FiveOfaKind,
    FourOfaKind,
    FullHouse,
    ThreeOfaKind,
    TwoPair,
    OnePair,
    HighCard,
}
// #[derive(Debug,Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    cards: String,
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    pub fn new(line: String) -> Self {
        let split_line = line.split_once(" ").unwrap();
        Self {
            cards: split_line.0.to_string(),
            bid: split_line.1.parse::<u32>().unwrap(),
            hand_type: Self::find_type(split_line.0),
        }
    }

    fn find_type(cards: &str) -> HandType {
        let mut chars = HashMap::new();

        for c in cards.chars() {
            let count = chars.entry(c).or_insert(0);
            *count += 1;
        }

        match chars
            .into_values()
            .sorted()
            .map(|n| n.to_string())
            .collect::<String>()
            .as_str()
        {
            "5" => return HandType::FiveOfaKind,
            "14" => return HandType::FourOfaKind,
            "23" => return HandType::FullHouse,
            "113" => return HandType::ThreeOfaKind,
            "122" => return HandType::TwoPair,
            "1112" => return HandType::OnePair,
            _ => return HandType::HighCard,
        }
    }

    fn primary_order(&self) -> u8 {
        match &self.hand_type {
            HandType::FiveOfaKind => 7,
            HandType::FourOfaKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfaKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Hand {{ Cards: {}, Bid: {}, Type: {:?}}}",
            self.cards, self.bid, self.hand_type
        )
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {

        (self.primary_order()).cmp(&other.primary_order())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        (self.bid) == (other.bid)
    }
}

impl Eq for Hand {}

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

// 32T3K
// T55J5
// KK677
// KTJJT
// QQQJA
