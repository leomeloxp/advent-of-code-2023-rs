fn main() {
    let file = include_str!("../../input.txt");
    println!("---------------------------");
    println!("Answer: {}", process_part1(file));
    println!("----------------------------");
}

fn process_part1(input: &str) -> String {
    let result = input
        .lines()
        .map(|line| {
            let game = line.split(": ").last().expect("No game found");
            let card_numbers = game
                .split(" | ")
                .next()
                .expect("No card data")
                .split(' ')
                .filter(|num| !num.is_empty());
            let drawn_numbers = game
                .split(" | ")
                .last()
                .expect("No draw data")
                .split(' ')
                .collect::<Vec<&str>>();
            let matched_numbers = card_numbers
                .clone()
                .filter(|num| drawn_numbers.contains(num))
                .count();

            if matched_numbers < 1 {
                return 0;
            }

            2_i32.pow(matched_numbers as u32 - 1)
        })
        .sum::<i32>()
        .to_string();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "13");
    }
}
