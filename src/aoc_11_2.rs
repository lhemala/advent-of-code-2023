use itertools::{enumerate, Itertools};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{
    alpha0, alpha1, anychar, char, digit1, line_ending, none_of, space1,
};
use nom::combinator::{complete, consumed, map_res, opt, recognize};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{pair, separated_pair};
use nom::IResult;
use rayon::iter::ParallelIterator;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator};
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::fmt::format;
use std::fs::DirEntry;
use std::ops::{Not, Range};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use strum::{EnumIter, FromRepr, IntoEnumIterator};

#[allow(unused)]
pub fn aoc_11_2(input: &str) {
    println!("aoc_11_2: {}", main(input, 1000000))
}

fn main(input: &str, expansion: usize) -> i64 {
    let positions = input
        .split_whitespace()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().fold(vec![], |mut acc, (x, char)| {
                if char == '#' {
                    acc.push(Position { x, y })
                }
                acc
            })
        })
        .collect::<Vec<_>>();

    #[cfg(test)]
    positions.iter().for_each(|p| println!("{p:?}"));

    let xs = positions.iter().map(|p| p.x).collect::<BTreeSet<_>>();
    let ys = positions.iter().map(|p| p.y).collect::<BTreeSet<_>>();

    let x_missing = get_missing(&xs);
    let y_missing = get_missing(&ys);

    #[cfg(test)]
    println!("x_missing: {x_missing:?}");
    #[cfg(test)]
    println!("y_missing: {y_missing:?}");

    let positions = positions
        .into_iter()
        .map(|pos| {
            let x_times = x_missing.iter().filter(|&&other| pos.x > other).count();
            let y_times = y_missing.iter().filter(|&&other| pos.y > other).count();

            Position {
                x: pos.x + x_times * (expansion - 1),
                y: pos.y + y_times * (expansion - 1),
            }
        })
        .collect::<Vec<_>>();

    #[cfg(test)]
    positions.iter().for_each(|p| println!("{p:?}"));

    let vecs = positions
        .into_iter()
        .tuple_combinations::<(_, _)>()
        .collect::<Vec<_>>();

    #[cfg(test)]
    vecs.iter().for_each(|p| println!("{p:?}"));

    println!("combinations: {}", vecs.len());

    vecs.iter()
        .map(|(a, b)| {
            let x = a.x.abs_diff(b.x);
            let y = a.y.abs_diff(b.y);
            (x + y) as i64
        })
        .sum()
}

fn get_missing(xs: &BTreeSet<usize>) -> Vec<usize> {
    (0usize..=xs.last().copied().unwrap())
        .collect::<BTreeSet<_>>()
        .difference(xs)
        .copied()
        .sorted()
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod test {
    use crate::aoc_11_2::main;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
        "
    };

    #[test]
    fn test_1() {
        assert_eq!(main(INPUT, 10), 1030);
    }

    #[test]
    fn test_2() {
        assert_eq!(main(INPUT, 100), 8410);
    }
}
