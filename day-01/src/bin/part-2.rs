fn main() {
    let file = include_str!("../../input.txt");
    println!("Answer: {}", process_part2(file));
}

fn process_part2(input: &str) -> String {
    let lines: Vec<String> = input
        .split('\n')
        .map(|line| {
            let line = line
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
        })
        .filter(|line| !line.is_empty())
        .collect();

    let digits: Vec<i32> = lines
        .iter()
        .map(|line| {
            let first_char = line.chars().next().unwrap();
            let last_char = line.chars().last().unwrap();
            format!("{}{}", first_char, last_char)
        })
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let sum = digits.iter().sum::<i32>();
    sum.to_string()
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
