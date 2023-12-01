fn main() {
    let file = include_str!("../../input.txt");
    println!("Answer: {}", process_part1(file));
}

fn process_part1(input: &str) -> String {
    let result = input
        .lines()
        .map(|line| line.chars().filter(|ch| ch.is_numeric()))
        .map(|line| {
            // We're using `clone()` here because if the line only has one digit
            // we need use it twice to generate the desired number.
            if let (Some(first_char), Some(last_char)) = (line.clone().next(), line.last()) {
                format!("{}{}", first_char, last_char)
                    .parse::<i32>()
                    .unwrap_or(0)
            } else {
                0
            }
        })
        .sum::<i32>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "142");
    }
}
