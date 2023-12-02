fn main() {
    let file = include_str!("../../input.txt");
    println!("---------------------------");
    println!("Answer: {}", process_part2(file));
    println!("----------------------------");
}

fn process_part2(_input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "");
    }
}
