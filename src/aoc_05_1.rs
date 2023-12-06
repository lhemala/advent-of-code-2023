use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha0, alpha1, anychar, char, digit1, line_ending, space1};
use nom::combinator::{complete, consumed, map_res, opt, recognize};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{pair, separated_pair};
use nom::IResult;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::DirEntry;
use std::ops::Range;

#[allow(unused)]
pub fn aoc_05_1(input: &str) {
    println!("aoc_05_1: {}", main(input))
}

fn main(input: &str) -> i64 {
    let (_, almanac) = Almanac::parse(input).unwrap();

    almanac.seeds.into_iter().fold(i64::MAX, |acc, seed| {
        println!("seed: {seed}");
        let mut current = seed;
        for map in &almanac.maps {
            current = map
                .ranges
                .iter()
                .find_map(|range| {
                    if range.source.contains(&current) {
                        Some(current + range.difference)
                    } else {
                        None
                    }
                })
                .unwrap_or(current);

            println!("{}: {current}", map.name);
        }
        min(acc, current)
    })
}

struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("seeds: ")(input)?;
        let (input, seeds) = separated_list1(space1, complete::i64)(input)?;
        let (input, _) = many1(line_ending)(input)?;

        let (input, maps) = separated_list1(line_ending, Map::parse)(input)?;

        Ok((input, Self { seeds, maps }))
    }
}

#[derive(Debug)]
struct Map {
    name: String,
    ranges: Vec<MapRange>,
}

impl Map {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, name) = recognize(separated_list1(tag("-"), alpha1))(input)?;
        let (input, _) = tag(" map:")(input)?;
        let (input, _) = line_ending(input)?;

        let (input, ranges) = many1(MapRange::parse)(input)?;

        let map = Map {
            name: name.to_string(),
            ranges,
        };
        println!("{map:?}");
        Ok((input, map))
    }
}

#[derive(Debug)]
struct MapRange {
    source: Range<i64>,
    difference: i64,
}

impl MapRange {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, destination) = complete::i64(input)?;
        let (input, _) = space1(input)?;
        let (input, source) = complete::i64(input)?;
        let (input, _) = space1(input)?;
        let (input, size) = complete::i64(input)?;
        let (input, _) = line_ending(input)?;

        Ok((
            input,
            Self {
                source: source..source + size,
                difference: destination - source,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::aoc_05_1::main;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        seeds: 79 14 55 13

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
        56 93 4"
    };

    #[test]
    fn test() {
        assert_eq!(main(INPUT), 35);
    }
}
