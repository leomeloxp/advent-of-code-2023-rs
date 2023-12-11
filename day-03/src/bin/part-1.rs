use std::vec;

fn main() {
    let file = include_str!("../../input.txt");
    println!("---------------------------");
    println!("Answer: {}", process_part1(file));
    println!("----------------------------");
}

#[derive(Debug, Clone)]
enum CellValue {
    Number(u32),
    Symbol(char),
}

#[derive(Debug, Clone)]
struct Cell {
    x: Vec<u32>,
    y: u32,
    value: CellValue,
}

struct Coord {
    x: i8,
    y: i8,
}

fn process_part1(input: &str) -> String {
    let adjacent_coords = vec![
        Coord { x: -1, y: -1 },
        Coord { x: 0, y: -1 },
        Coord { x: 1, y: -1 },
        Coord { x: -1, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: -1, y: 1 },
        Coord { x: 0, y: 1 },
        Coord { x: 1, y: 1 },
    ];

    let (numbers, symbols): (Vec<Cell>, Vec<Cell>) =
        input_to_cells(input)
            .iter()
            .fold((vec![], vec![]), |mut acc, cell| {
                match cell.value {
                    CellValue::Number(_) => acc.0.push(cell.clone()),
                    CellValue::Symbol(_) => acc.1.push(cell.clone()),
                }

                acc
            });

    dbg!(symbols, numbers);
    "".to_string()
}

fn input_to_cells(input: &str) -> Vec<Cell> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, char)| {
                if char == '.' {
                    return None;
                }

                match char {
                    num if num.is_numeric() => {
                        let value =
                            CellValue::Number(num.to_digit(10).expect("Should be a number"));
                        Some(Cell {
                            x: vec![x as u32],
                            y: y as u32,
                            value,
                        })
                    }
                    _ => {
                        let value = CellValue::Symbol(char);
                        Some(Cell {
                            x: vec![x as u32],
                            y: y as u32,
                            value,
                        })
                    }
                }
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "4361");
    }
}
