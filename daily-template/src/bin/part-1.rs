fn main() {
    let file = include_str!("../../input.txt");
    println!("---------------------------");
    println!("Answer: {}", process_part1(file));
    println!("----------------------------");
}

fn process_part1(_input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "");
    }
}
