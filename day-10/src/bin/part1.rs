#![allow(unused)]

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    "todo!()".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        );
        assert_eq!(result, "8".to_string());

        let result = part1(
            "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        );
        assert_eq!(result, "4".to_string());
    }
}
