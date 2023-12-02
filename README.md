# Advent of Code 2023
Learning some rust by struggling through problems.

## Handy Links
[Advent of code 2023](https://adventofcode.com/2023)

[How to set up Rust for Advent of Code](https://www.youtube.com/watch?v=fEQv-cqzbPg)

[GitIgnore for Rust](https://github.com/rust-lang/cargo/blob/master/.gitignore)

## Starting Boilerplate

```rust
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(_input:&str) -> String {
    "todo!()".to_string()
}

#[cfg(test)]
mod tests {
    //use crate::part1;
    use super::*;

    #[test]
    fn exploration() {
        let result = part1("");
        assert_eq!(result, "4".to_string());
    }
}
```