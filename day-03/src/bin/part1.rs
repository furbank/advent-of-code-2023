use regex::{Match, Regex};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut num_map: Vec<Component> = Vec::new();
    let mut symbol_map: Vec<Component> = Vec::new();

    for line in input.lines().enumerate() {
        for m in regex_find_numbers(line.1) {
            num_map.push(Component::new(m, line.0));
        }

        for m in regex_find_symbols(line.1) {
            symbol_map.push(Component::new(m, line.0));
        }
    }

    let mut total: i32 = 0;
    for nm in num_map {
        for sym in &symbol_map {
            if nm.neighbours(sym.line, sym.start) {
                total += nm.value_int();
                break;
            }
        }
    }

    total.to_string()
}

#[derive(Debug)]
struct Component {
    value: String,
    line: i32,
    start: i32,
    end: i32,
}

impl Component {
    pub fn new(m: Match, ln: usize) -> Self {
        Self {
            value: m.as_str().to_string(),
            line: ln as i32,
            start: m.start() as i32,
            end: m.end() as i32,
        }
    }

    pub fn neighbours(&self, line: i32, start: i32) -> bool {
        if line >= self.line - 1 && line <= self.line + 1 {
            if (self.start - 1..self.end + 1).contains(&start) {
                return true;
            }
        }

        return false;
    }

    pub fn value_int(&self) -> i32 {
        self.value.parse().unwrap()
    }
}

fn regex_find_numbers(s: &str) -> Vec<Match> {
    let rg: Regex = Regex::new(r"\d+").unwrap();

    // iterate over all matches
    rg.find_iter(s).collect()
}

fn regex_find_symbols(s: &str) -> Vec<Match> {
    let rg: Regex = Regex::new(r"[^.0-9\s]").unwrap();

    // iterate over all matches
    rg.find_iter(s).collect()
}

#[cfg(test)]
mod tests {
    //use crate::part1;
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, "4361".to_string());
    }
}
