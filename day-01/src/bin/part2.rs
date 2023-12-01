#![allow(unused)]

use std::string;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let mut total: u32 = 0;

    let lines: Vec<String> = input.lines().map(String::from).collect();
    for l in &lines {
        let first_num: String = find_first_number(l);
        let last_num: String = find_last_number(l);

        println!("Line: {}, First {}, Last {}", l, first_num, last_num);
        let num: String = format!("{}{}", first_num, last_num);
        let int: u32 = num.parse().unwrap();
        total += int
    }
    total
}

fn find_first_number(line: &String) -> String {
    let mut candidates: Vec<(u32, String)> = Vec::new();

    //Look for first numeral
    match line.find(|c: char| c.is_numeric()) {
        Some(result) => {
            let num_poss = u32::try_from(result).unwrap();
            candidates.push((num_poss, line.chars().nth(result).unwrap().to_string()));
        }
        _ => (),
    }

    //Look for spelt numbers
    let numbers: [(&str, String); 9] = [
        ("one", "1".to_string()),
        ("two", "2".to_string()),
        ("three", "3".to_string()),
        ("four", "4".to_string()),
        ("five", "5".to_string()),
        ("six", "6".to_string()),
        ("seven", "7".to_string()),
        ("eight", "8".to_string()),
        ("nine", "9".to_string()),
    ];

    for n in numbers {
        match line.find(n.0) {
            Some(result) => {
                let num_poss = u32::try_from(result).unwrap();
                candidates.push((num_poss, n.1));
            }
            _ => (),
        };
    }

    // println!("{:?}", candidates);

    // Sort candidates
    candidates.sort_by(|a, b| a.0.cmp(&b.0)); // increasing order

    // if candidates.len() > 0 {
    //     println!(
    //         "Line: {}, Possition {}, Value {}, {:?}",
    //         line, candidates[0].0, candidates[0].1, candidates
    //     );
    // }

    // Return first
    candidates[0].1.to_string()
}

fn find_last_number(line: &String) -> String {
    let mut candidates: Vec<(u32, String)> = Vec::new();

    //Look for last numeral
    match line.rfind(|c: char| c.is_numeric()) {
        Some(result) => {
            let num_poss = u32::try_from(result).unwrap();
            candidates.push((num_poss, line.chars().nth(result).unwrap().to_string()));
        }
        _ => (),
    }

    //Look for spelt numbers
    let numbers: [(&str, String); 9] = [
        ("one", "1".to_string()),
        ("two", "2".to_string()),
        ("three", "3".to_string()),
        ("four", "4".to_string()),
        ("five", "5".to_string()),
        ("six", "6".to_string()),
        ("seven", "7".to_string()),
        ("eight", "8".to_string()),
        ("nine", "9".to_string()),
    ];

    for n in numbers {
        match line.rfind(n.0) {
            Some(result) => {
                let num_poss = u32::try_from(result).unwrap();
                candidates.push((num_poss, n.1));
            }
            _ => (),
        };
    }

    // println!("{:?}", candidates);

    // Sort candidates
    candidates.sort_by(|a, b| b.0.cmp(&a.0)); // decreasing order

    // if candidates.len() > 0 {
    //     println!(
    //         "Line: {}, Possition {}, Value {}, {:?}",
    //         line, candidates[0].0, candidates[0].1, candidates
    //     );
    // }

    // Return last
    candidates[0].1.to_string()
}

#[cfg(test)]
mod tests {
    //use crate::part2;
    use super::*;

    #[test]
    fn all_sample_data() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );

        assert_eq!(result, 281);
    }
}
