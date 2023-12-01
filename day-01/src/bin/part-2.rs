fn main() {
    let file = include_str!("../../input.txt");
    println!("Answer: {}", process_part2(file));
}

fn process_part2(input: &str) -> String {
    let lines_iter = input.lines().map(|line| {
        let line = line
            // Replacing the words with the numbers in the middle
            // to make it easier to extract the numbers later.
            .replace("one", "on1e")
            .replace("two", "tw2o")
            .replace("three", "th3ree")
            .replace("four", "fo4ur")
            .replace("five", "fi5ve")
            .replace("six", "si6x")
            .replace("seven", "se7ven")
            .replace("eight", "ei8ght")
            .replace("nine", "ni9ne");

        line.chars()
            .filter(|ch| ch.is_numeric())
            .collect::<String>()
    });

    let result: i32 = lines_iter
        .map(|line| {
            // We're using `clone()` here because if the line only has one digit
            // we need use it twice to generate the desired number.
            if let (Some(first_char), Some(last_char)) = (line.chars().next(), line.chars().last())
            {
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

    const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "281");
    }
}
