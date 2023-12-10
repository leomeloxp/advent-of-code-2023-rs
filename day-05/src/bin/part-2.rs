use indicatif::ParallelProgressIterator;
use std::ops::Range;

use nom::{
    bytes::complete::take_until,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let file = include_str!("../../input.txt");
    println!("---------------------------");
    println!("Answer: {}", process_part2(file));
    println!("----------------------------");
}

fn process_part2(input: &str) -> String {
    let (_, (seeds, maps)) = parse_seeds_info(input).expect("parse to work");

    let count = seeds
        .clone()
        .into_par_iter()
        .flat_map(|seed_range| seed_range.clone())
        .count() as u64;

    let locations = seeds
        .into_par_iter()
        .flat_map(|seed_range| seed_range.clone())
        .progress_count(count)
        .map(|seed| {
            maps.iter()
                .fold(seed, |seed, map| map.traverse_mapping(seed))
        })
        .collect::<Vec<u64>>();

    locations
        .iter()
        .min()
        .expect("should have a minimum location value")
        .to_string()
}

#[derive(Debug)]
struct SeedMap {
    mappings: Vec<Mapping>,
}

#[derive(Debug)]
struct Mapping {
    source_range: Range<u64>,
    destination_range: Range<u64>,
}

impl SeedMap {
    fn traverse_mapping(&self, source: u64) -> u64 {
        let valid_source_mapping = self
            .mappings
            .iter()
            .find(|mapping| mapping.source_range.contains(&source));

        let Some(mapping) = valid_source_mapping else {
            return source;
        };

        let offset = source - mapping.source_range.start;

        mapping.destination_range.start + offset
    }
}

fn parse_seeds_info(input: &str) -> IResult<&str, (Vec<Range<u64>>, Vec<SeedMap>)> {
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(
            space1,
            separated_pair(complete::u64, tag(" "), complete::u64)
                .map(|(start, length)| start..(start + length)),
        ))
        .parse(input)?;

    let (input, maps) = many1(parse_maps_info)(input)?;

    Ok((input, (seeds, maps)))
}

fn parse_maps_info(input: &str) -> IResult<&str, SeedMap> {
    take_until("map:")
        .precedes(tag("map:"))
        .precedes(
            many1(line_ending.precedes(parse_range_line)).map(|mappings| SeedMap { mappings }),
        )
        .parse(input)
}

fn parse_range_line(input: &str) -> IResult<&str, Mapping> {
    let res: IResult<_, (u64, u64, u64)> = tuple((
        complete::u64,
        complete::u64.preceded_by(tag(" ")),
        complete::u64.preceded_by(tag(" ")),
    ))(input);

    let (input, (destination, source, length)) = res?;

    Ok((
        input,
        Mapping {
            source_range: source..(source + length),
            destination_range: destination..(destination + length),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "46");
    }
}
