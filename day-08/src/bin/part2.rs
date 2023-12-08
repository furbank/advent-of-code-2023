#![allow(unused)]
use std::collections::HashMap;

use std::thread::current;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let instructions: Vec<char> = input.lines().nth(0).unwrap().chars().collect::<Vec<char>>();
    let mut nodes: HashMap<String, Node> = HashMap::new();

    for line in input.lines().skip(2) {
        nodes.insert(
            line.split_once(" ").unwrap().0.to_string().to_owned(),
            Node::new(line),
        );
    }

    println!("Instruction Line: {:?}", instructions);
    // for nkey in nodes.keys() {
    //     println!(
    //         "Node name: {} Left: {:?} Right: {:?}",
    //         nkey,
    //         nodes[nkey].next_node("L").unwrap(),
    //         nodes[nkey].next_node("R").unwrap()
    //     );
    // }

    let mut steps = 0;
    let mut current_node = "AAA";
    let mut next_node: &str;

    while current_node != "ZZZ" {
        for i in &instructions {
            next_node = nodes[current_node].next_node(&i).unwrap();
            steps += 1;

            println!(
                "Step: {}, Instruction: {}, {} >> {}",
                steps, i, &current_node, &next_node
            );
            current_node = next_node;
        }

        if steps % instructions.len() == 20 {
            panic!("WHILE looped more than 20 times!")
        }
    }

    steps.to_string()
}

#[derive(Debug)]
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
}
#[cfg(test)]
mod tests {
    //use crate::part1;
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, "6".to_string());
    }
}
