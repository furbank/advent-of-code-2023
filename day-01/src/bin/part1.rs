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
