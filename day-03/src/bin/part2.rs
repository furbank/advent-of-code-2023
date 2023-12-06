use regex::{Match, Regex};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut num_map: Vec<Component> = Vec::new();
    let mut gear_map: Vec<Gear> = Vec::new();

    for line in input.lines().enumerate() {
        for m in regex_find_numbers(line.1) {
            num_map.push(Component::new(m, line.0));
        }

        for m in regex_find_stars(line.1) {
            gear_map.push(Gear::new(m, line.0));
        }
    }

    for g in &mut gear_map {
        for n in &num_map {
            if n.neighbours(g.line, g.start) {
                g.add_neighbour(n.clone())
            }
        }
    }

    let mut total: i32 = 0;
    for g in &gear_map {
        if g.neigbours.len() == 2 {
            let p1 = g.neigbours[0].value_int();
            let p2 = g.neigbours[1].value_int();
            total += p1 * p2;
        }
    }

    total.to_string()
}

#[derive(Debug, Clone)]
struct Component {
    value: String,
    line: i32,
    start: i32,
    end: i32,
}

impl Component {
    pub fn new(m: Match, ln: usize) -> Self {
        // m.start().try_into();
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
        let value_i32: i32 = self.value.parse().unwrap();
        value_i32
    }
}

#[derive(Debug)]
struct Gear {
    line: i32,
    start: i32,
    neigbours: Vec<Component>,
}

impl Gear {
    pub fn new(m: Match, ln: usize) -> Self {
        Self {
            line: ln as i32,
            start: m.start() as i32,
            neigbours: Vec::new(),
        }
    }

    pub fn add_neighbour(&mut self, part: Component) {
        // Prevent duplicates
        if self.neigbours.is_empty() == false {
            // Compare to existing neigbours
            for n in &self.neigbours {
                if n.start == part.start && n.line == part.line {
                    return;
                }
            }
        }

        self.neigbours.push(part);
    }
}

fn regex_find_numbers(s: &str) -> Vec<Match> {
    let rg: Regex = Regex::new(r"\d+").unwrap();

    // iterate over all matches
    rg.find_iter(s).collect()
}

fn regex_find_stars(s: &str) -> Vec<Match> {
    let rg: Regex = Regex::new(r"[*]").unwrap();

    // iterate over all matches
    rg.find_iter(s).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
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
        assert_eq!(result, "467835".to_string());
    }
}
