use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha0, alpha1, anychar, char, digit1, line_ending, space1};
use nom::combinator::{complete, consumed, map_res, opt, recognize};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{pair, separated_pair};
use nom::IResult;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::DirEntry;
use std::ops::Range;

#[allow(unused)]
pub fn aoc_06_2(input: &str) {
    println!("aoc_06_2: {}", main(input))
}

fn main(input: &str) -> i64 {
    let (_, race) = Race::parse(input).unwrap();

    println!("{race:?}");

    race.calc_options()
}

#[derive(Debug)]
struct Race {
    time_limit: i64,
    distance_to_beat: i64,
}

impl Race {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Time:")(input)?;
        let (input, _) = space1(input)?;
        let (input, time_limits) = separated_list1(space1, digit1)(input)?;
        let (input, _) = many1(line_ending)(input)?;
        let (input, _) = tag("Distance:")(input)?;
        let (input, _) = space1(input)?;
        let (input, distances_to_beat) = separated_list1(space1, digit1)(input)?;

        Ok((
            input,
            Self {
                time_limit: time_limits.join("").parse().unwrap(),
                distance_to_beat: distances_to_beat.join("").parse().unwrap(),
            },
        ))
    }

    fn calc_options(self) -> i64 {
        (1..=self.time_limit)
            .collect::<Vec<_>>()
            .into_iter()
            .map(|button_time| {
                let speed = button_time;
                let time_for_moving = self.time_limit - button_time;
                let distance = time_for_moving * speed;
                distance > self.distance_to_beat
            })
            .filter(|b| *b)
            .count() as i64
    }
}

#[cfg(test)]
mod test {
    use crate::aoc_06_2::main;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200"
    };

    #[test]
    fn test() {
        assert_eq!(main(INPUT), 71503);
    }
}
