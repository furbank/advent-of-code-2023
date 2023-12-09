#![allow(unused)]

use num::integer::lcm;
use num::Integer;
use std::collections::HashMap;
use std::thread::current;
use std::{default, string};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let instructions: Vec<char> = input.lines().nth(0).unwrap().chars().collect::<Vec<char>>();

    let mut nodes: HashMap<String, Node> = HashMap::new();

    for line in input.lines().skip(2) {
        nodes.insert(
            line.split_once(" ").unwrap().0.to_string().to_owned(),
            Node::new(line),
        );
    }

    let mut routes: Vec<Route> = Vec::new();

    for nkey in nodes.keys() {
        if nodes[nkey].start_node() {
            routes.push(Route::new(nodes[nkey].name.to_string(), nodes.clone()));
        }
    }

    println!("Start Nodes:");
    for route in &routes {
        println!("{:?}", route.start);
    }

    let mut loops: usize = 0;
    let mut all_end: bool = false;

    loop {
        for i in &instructions {
            for r in &mut routes {
                r.step(*i)
            }

            all_end = routes[0].at_end().clone();
            for r in &routes[1..] {
                all_end = all_end && r.at_end();
            }
        }

        if all_end {
            break;
        }

        loops += 1;
        if loops >= 200 {
            println!("Break loop @ {}", loops);
            break;
        }
    }

    let mut lcm_steps: usize = routes[0].min_steps;
    println!("{}, {}", routes[0].start, lcm_steps);

    for r in routes.iter().skip(1) {
        print!("{}: {} ", r.start.to_string(), r.min_steps);
        lcm_steps = lcm_steps.lcm(&r.min_steps);
        println!("{}", lcm_steps);
    }

    lcm_steps.to_string()
}

#[derive(Debug, Clone)]
struct Route {
    start: String,
    current: String,
    nodes: HashMap<String, Node>,
    ends: HashMap<usize, String>,
    steps: usize,
    min_steps: usize,
}

impl Route {
    fn new(start: String, nodes: HashMap<String, Node>) -> Self {
        Self {
            start: start.to_string(),
            current: start.to_string(),
            nodes: nodes,
            ends: HashMap::new(),
            steps: 0,
            min_steps: 0,
        }
    }

    pub fn step(&mut self, instruction: char) {
        self.current = self.nodes[&self.current]
            .next_node(&instruction)
            .unwrap()
            .to_string();

        self.steps += 1;

        if self.at_end() {
            self.record_end();
        };
    }

    fn at_end(&self) -> bool {
        self.nodes[&self.current].end_node()
    }

    fn record_end(&mut self) {
        if self.min_steps == 0 {
            self.min_steps = self.steps.clone();
        }

        self.ends.insert(self.steps, self.current.to_string());
    }

    pub fn show_ends(self) {
        println!("Start: {}, total steps: {}", self.start, self.steps);
        println!("found {} ends", self.ends.len());
        println!("{:#?}", self.ends)
    }
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn new(line: &str) -> Self {
        let clean_line = line.replace("= (", "").replace(",", "").replace(")", "");
        let mid = clean_line.split_once(" ").unwrap();

        Self {
            name: mid.0.to_string(),
            left: mid.1.to_string().split_once(" ").unwrap().0.to_string(),
            right: mid.1.to_string().split_once(" ").unwrap().1.to_string(),
        }
    }

    pub fn next_node(&self, instruction: &char) -> Option<&str> {
        match instruction {
            'L' => Some(&self.left),
            'R' => Some(&self.right),
            _ => None,
        }
    }

    pub fn start_node(&self) -> bool {
        self.name.ends_with("A")
    }

    pub fn end_node(&self) -> bool {
        self.name.ends_with("Z")
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#,
        );
        assert_eq!(result, "6".to_string());
    }
}
