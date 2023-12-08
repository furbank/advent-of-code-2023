#![allow(unused)]

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let instructions: String = input.lines().nth(0).unwrap().to_string();
    let nodes: Vec<Node> = input.lines().skip(2).map(|l| Node::new(l)).collect();

    println!("Instruction Line: {}", instructions);
    for n in nodes{
        println!("{:?}", n);
    }
    "todo!()".to_string()
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
}
#[cfg(test)]
mod tests {
    //use crate::part1;
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, "6".to_string());
    }
}
