fn main() {
    let file = include_str!("../../input.txt");
    println!("Answer: {}", process_part1(file));
}

fn process_part1(input: &str) -> String {
    let lines: Vec<String> = input
        .split('\n')
        .map(|line| {
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
