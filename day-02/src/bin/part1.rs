fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!("Sum of valid games: {}", output);
}

fn part1(input: &str) -> u32 {
    let mut games: Vec<Game> = Vec::new();

    for l in input.lines() {
        games.push(Game::new(l.to_string()));
    }

    // dbg!(games.len());
    // for game in games {
    //     game.show();
    //     println!("Max red: {}", game.max_red());
    //     println!("Max green: {}", game.max_green());
    //     println!("Max blue: {}", game.max_blue());
    // }

    // limits: 12 red, 13 green, 14 blue
    let mut sum: u32 = 0;
    let limit_red: u32 = 12;
    let limit_green: u32 = 13;
    let limit_blue: u32 = 14;

    for game in games {
        if game.max_red() <= limit_red
            && game.max_green() <= limit_green
            && game.max_blue() <= limit_blue
        {
            sum += game.id
        }
    }
    sum
}

pub struct Game {
    id: u32,
    all_rounds: String,
    rounds: Vec<Round>,
}

impl Game {
    pub fn new(line: String) -> Self {
        let parts = line.strip_prefix("Game ").unwrap().split_once(": ");
        let all_rounds = parts.unwrap().1.to_string();

        let mut rounds: Vec<Round> = Vec::new();
        for r in all_rounds.split(";") {
            rounds.push(Round::new(r.trim().to_string()));
        }

        Self {
            id: parts.unwrap().0.parse().unwrap(),
            all_rounds: all_rounds,
            rounds: rounds,
        }
    }

    pub fn show(&self) {
        println!("id: {}, rounds: {}", self.id, self.all_rounds);
        println!("Total rounds: {}", self.rounds.len());
        for round in &self.rounds {
            round.show();
        }
    }

    pub fn max_red(&self) -> u32 {
        let mut max_red: u32 = 0;
        for round in &self.rounds {
            if max_red < round.red {
                max_red = round.red;
            }
        }
        max_red
    }

    pub fn max_green(&self) -> u32 {
        let mut max_green: u32 = 0;
        for round in &self.rounds {
            if max_green < round.green {
                max_green = round.green;
            }
        }
        max_green
    }

    pub fn max_blue(&self) -> u32 {
        let mut max_blue: u32 = 0;
        for round in &self.rounds {
            if max_blue < round.blue {
                max_blue = round.blue;
            }
        }
        max_blue
    }
}

pub struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    pub fn new(rline: String) -> Self {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        for r in rline.split(",") {
            let colour = r.trim().split_once(" ");
            match colour.unwrap().1 {
                "red" => red = colour.unwrap().0.parse::<u32>().unwrap(),
                "green" => green = colour.unwrap().0.parse::<u32>().unwrap(),
                "blue" => blue = colour.unwrap().0.parse::<u32>().unwrap(),
                _ => (),
            }
        }

        Self {
            red: red,
            green: green,
            blue: blue,
        }
    }

    pub fn show(&self) {
        println!(
            "Red: {}, Green {}, Blue {}",
            self.red, self.green, self.blue
        )
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
        assert_eq!(result, 8);
    }
}
