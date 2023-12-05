use std::collections::{BTreeMap, HashSet};

fn main() {
    let file = include_str!("../../input.txt");
    println!("---------------------------");
    println!("Answer: {}", process_part2(file));
    println!("----------------------------");
}

fn process_part2(input: &str) -> String {
    let cards_data = input
        .lines()
        .map(ScratchCard::from_string)
        .map(|card| card.matched_numbers)
        .collect::<Vec<_>>();

    let init = (0..cards_data.len())
        .map(|index| (index, 1))
        .collect::<BTreeMap<usize, u32>>();

    let result = cards_data
        .iter()
        .enumerate()
        .fold(init, |mut acc, (index, matched_number)| {
            let to_add = *acc.get(&index).expect("should exist");

            for i in (index + 1)..(index + 1 + *matched_number as usize) {
                acc.entry(i).and_modify(|value| {
                    *value += to_add;
                });
            }
            acc
        })
        .values()
        .sum::<u32>();

    result.to_string()
}

#[derive(Debug, Clone)]
struct ScratchCard {
    matched_numbers: u32,
}

impl ScratchCard {
    fn from_string(str: &str) -> Self {
        let mut parts = str
            .split(':')
            .last()
            .expect("Scratch Card should have numbers")
            .split(" | ");

        fn string_to_hash_set(input: &str) -> HashSet<u32> {
            input
                .split(' ')
                .filter_map(|num| {
                    if let Ok(int) = num.parse::<u32>() {
                        Some(int)
                    } else {
                        None
                    }
                })
                .collect()
        }

        let winning_numbers =
            string_to_hash_set(parts.next().expect("Should have winning numbers string"));
        let scratched_numbers =
            string_to_hash_set(parts.next().expect("Should have winning numbers string"));

        ScratchCard {
            matched_numbers: winning_numbers.intersection(&scratched_numbers).count() as u32,
        }
    }
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
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "30");
    }
}
