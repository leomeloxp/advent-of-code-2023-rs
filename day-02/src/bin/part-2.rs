fn main() {
    let file = include_str!("../../input.txt");
    println!("Answer: {}", process_part2(file));
}

fn process_part2(input: &str) -> String {
    println!("Processing Part 2");

    let games_iter = input.lines().map(|line| {
        let mut split_iter = line.split(": ");
        let game_id_str = split_iter.next().unwrap();
        let rounds_split = split_iter
            .next()
            .unwrap()
            .split("; ")
            .map(CubeCount::from_string)
            .collect::<Vec<CubeCount>>();

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
        .map(|game| {
            let minimum_cubes = game.minimum_cubes();

            minimum_cubes.blue * minimum_cubes.green * minimum_cubes.red
        })
        .sum::<u32>();

    result.to_string()
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<CubeCount>,
}

impl Game {
    fn minimum_cubes(&self) -> CubeCount {
        let green_max = self
            .rounds
            .iter()
            .map(|round| round.green)
            .max()
            .expect("Should be a number");

        let blue_max = self
            .rounds
            .iter()
            .map(|round| round.blue)
            .max()
            .expect("Should be a number");

        let red_max = self
            .rounds
            .iter()
            .map(|round| round.red)
            .max()
            .expect("Should be a number");

        CubeCount {
            green: green_max,
            blue: blue_max,
            red: red_max,
        }
    }
}

#[derive(Debug)]
struct CubeCount {
    green: u32,
    blue: u32,
    red: u32,
}

impl CubeCount {
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
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "2286");
    }
}
