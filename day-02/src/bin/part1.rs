fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut games: Vec<Game> = Vec::new();

    let lines: Vec<String> = input.lines().map(String::from).collect();
    for l in &lines {
        games.push(Game::new(l.to_string()));
    }

    dbg!(games.len());
    games[0].show();
    games[99].show();

    "todo!()".to_string()
}

pub struct Game {
    id: u32,
    all_rounds: String,
}

impl Game {
    fn new(line: String) -> Self {
        let parts = line.strip_prefix("Game ").unwrap().split_once(": ");
        Self {
            id: parts.unwrap().0.parse().unwrap(),
            all_rounds: parts.unwrap().1.to_string(),
        }
    }

    fn show(&self) {
        println!("id: {}, rounds: {}", self.id, self.all_rounds);
    }
}

#[cfg(test)]
mod tests {
    //use crate::part1;
    use super::*;

    #[test]
    fn exploration() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "8".to_string());
    }
}
