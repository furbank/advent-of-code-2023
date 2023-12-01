fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut total: u32 = 0;

    let lines: Vec<String> = input.lines().map(String::from).collect();
    for l in &lines {
        let d: Vec<char> = l.chars().filter(|c| c.is_digit(10)).collect();
        let num: String = format!("{}{}", d[0], d[d.len() - 1]);
        let int: u32 = num.parse().unwrap();
        total += int
    }
    total
}

#[cfg(test)]
mod tests {
    //use crate::part1;
    use super::*;

    #[test]
    fn exploration() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );

        assert_eq!(result, 142);
    }
}
