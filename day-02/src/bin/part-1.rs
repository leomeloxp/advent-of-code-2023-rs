fn main() {
    let file = include_str!("../../input.txt");
    println!("Answer: {}", process_part1(file));
}

fn process_part1(input: &str) -> String {
    println!("Processing Part 1");

    let games_iter = input.lines().map(|line| {
        let mut split_iter = line.split(": ");
        let game_id_str = split_iter.next().unwrap();
        let rounds_split = split_iter
            .next()
            .unwrap()
            .split("; ")
            .map(Round::from_string)
            .collect::<Vec<Round>>();

        let game_id = game_id_str
            .split(' ')
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        Game {
            id: game_id,
            rounds: rounds_split,
        }
    });

    let result = games_iter
        .filter_map(|game| {
            if game.is_possible(14, 13, 12) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum::<u32>();

    result.to_string()
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn is_possible(&self, blue_limit: u32, green_limit: u32, red_limit: u32) -> bool {
        self.rounds.iter().all(|round| {
            round.blue <= blue_limit && round.green <= green_limit && round.red <= red_limit
        })
    }
}

#[derive(Debug)]
struct Round {
    green: u32,
    blue: u32,
    red: u32,
}

impl Round {
    fn from_string(str: &str) -> Self {
        let mut green = 0;
        let mut blue = 0;
        let mut red = 0;
        let round = str.split(", ");
        for s in round {
            let mut split = s.split(' ');
            let count = split.next().unwrap().parse::<u32>().unwrap();
            let color = split.next().unwrap();
            match color {
                "green" => green = count,
                "blue" => blue = count,
                "red" => red = count,
                _ => {}
            }
        }

        Self { green, blue, red }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "8");
    }
}
