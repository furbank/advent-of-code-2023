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

    all_hands.sort_by(|a, b| a.cmp(b));

    let total: usize = all_hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| ((i + 1) * h.bid))
        .sum::<usize>();

        total.to_string()
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
    bid: usize,
    hand_type: HandType,
}

impl Hand {
    pub fn new(line: String) -> Self {
        let split_line = line.split_once(" ").unwrap();
        Self {
            cards: split_line.0.to_string(),
            bid: split_line.1.parse::<usize>().unwrap(),
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

    pub fn card_order(&self) -> u64 {
        return format!("{}{}", self.primary_order(), self.secondary_order())
            .parse::<u64>()
            .unwrap();
    }

    fn primary_order(&self) -> String {
        match &self.hand_type {
            HandType::FiveOfaKind => "7".to_string(),
            HandType::FourOfaKind => "6".to_string(),
            HandType::FullHouse => "5".to_string(),
            HandType::ThreeOfaKind => "4".to_string(),
            HandType::TwoPair => "3".to_string(),
            HandType::OnePair => "2".to_string(),
            HandType::HighCard => "1".to_string(),
        }
    }

    fn secondary_order(&self) -> String {
        return self
            .cards
            .chars()
            .map(|c| Self::card_weight(c))
            .collect::<String>();
    }

    fn card_weight(card: char) -> String {
        match card {
            'A' => "13".to_string(),
            'K' => "12".to_string(),
            'Q' => "11".to_string(),
            'J' => "10".to_string(),
            'T' => "09".to_string(),
            '9' => "08".to_string(),
            '8' => "07".to_string(),
            '7' => "06".to_string(),
            '6' => "05".to_string(),
            '5' => "04".to_string(),
            '4' => "03".to_string(),
            '3' => "02".to_string(),
            '2' => "01".to_string(),
            _ => "".to_string(),
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
        (self.card_order()).cmp(&other.card_order())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        (self.card_order()) == (other.card_order())
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
