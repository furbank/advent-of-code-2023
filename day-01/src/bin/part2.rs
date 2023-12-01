fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let mut total: u32 = 0;

    let lines: Vec<String> = input.lines().map(String::from).collect();
    for line in &lines {
        let l = replace_numbers(line);
        println!("{} --> {}", line, l);
        let d: Vec<char> = l.chars().filter(|c| c.is_digit(10)).collect();
        let num: String = format!("{}{}", d[0], d[d.len() - 1]);
        let int: u32 = num.parse().unwrap();
        total += int
    }
    total
}

fn replace_numbers(line: &String) -> String {
    let mut newline = line.replace("one", "1");
    newline = newline.replace("two", "2");
    newline = newline.replace("three", "3");
    newline = newline.replace("four", "4");
    newline = newline.replace("five", "5");
    newline = newline.replace("six", "6");
    newline = newline.replace("seven", "7");
    newline = newline.replace("eight", "8");
    newline = newline.replace("nine", "9");

    newline.to_string()
}


#[cfg(test)]
mod tests {
    //use crate::part2;
    use super::*;

    #[test]
    fn exploration() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );

        assert_eq!(result, 281);
    }
}
