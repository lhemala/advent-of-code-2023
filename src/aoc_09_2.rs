use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{
    alpha0, alpha1, anychar, char, digit1, line_ending, none_of, space1,
};
use nom::combinator::{complete, consumed, map_res, opt, recognize};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{pair, separated_pair};
use nom::IResult;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::fs::DirEntry;
use std::ops::Range;
use strum::{EnumIter, FromRepr, IntoEnumIterator};

#[allow(unused)]
pub fn aoc_09_2(input: &str) {
    println!("aoc_09_2: {}", main(input))
}

fn main(input: &str) -> i64 {
    let (_, sequences) = many1(Sequence::parse)(input).unwrap();

    #[cfg(test)]
    println!("{sequences:?}");

    sequences.into_iter().map(|sq| sq.extrapolate()).sum()
}

#[derive(Debug)]
struct Sequence {
    values: Vec<i64>,
}

impl Sequence {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, values) = separated_list1(space1, complete::i64)(input)?;
        let (input, _) = opt(line_ending)(input)?;

        Ok((
            input,
            Self {
                values: values.into_iter().rev().collect(),
            },
        ))
    }

    fn extrapolate(self) -> i64 {
        let mut diffs = vec![self.values.clone()];

        let mut vals = self.values;
        loop {
            vals = vals
                .iter()
                .tuple_windows()
                .map(|(a, b)| a - b)
                .collect::<Vec<_>>();

            let sum: i64 = vals.iter().sum();

            #[cfg(test)]
            println!("{vals:?}");

            diffs.push(vals.clone());

            if sum == 0 {
                break;
            }
        }

        let (_, extras) =
            diffs
                .into_iter()
                .rev()
                .fold((0i64, vec![]), |(last, mut extrapolated), sequence| {
                    let next = sequence.last().unwrap() - last;
                    extrapolated.push(next);
                    (next, extrapolated)
                });

        #[cfg(test)]
        println!("extras: {extras:?}");

        extras.last().cloned().unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::aoc_09_2::main;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
        "
    };

    #[test]
    fn test() {
        assert_eq!(main(INPUT), 2);
    }
}
