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
pub fn aoc_11_1(input: &str) {
    println!("aoc_11_1: {}", main(input))
}

fn main(input: &str) -> i64 {
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
                x: pos.x + x_times,
                y: pos.y + y_times,
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

    let done = AtomicUsize::new(0);

    vecs.par_iter()
        .map(|(a, b)| {
            let x = a.get_shortest_path(b) as i64;
            println!("{}", done.fetch_add(1, SeqCst));
            x
        })
        .sum()
}

fn get_missing(xs: &BTreeSet<usize>) -> Vec<usize> {
    (0usize..=*xs.last().unwrap())
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

impl Position {
    fn get_shortest_path(&self, target: &Self) -> usize {
        let mut open = BinaryHeap::new();
        let mut closed = HashSet::new();

        open.push(Node {
            position: self.clone(),
            h: self.get_distance(target),
            g: 0,
        });

        #[cfg(test)]
        println!("{open:?}");

        loop {
            let current = open.pop().unwrap();

            if &current.position == target {
                #[cfg(test)]
                println!("reached target");
                return current.g;
            }

            #[cfg(test)]
            println!("closing pos {:?}", current.position);
            closed.insert(current.position.clone());

            for position in current.position.get_surrounding() {
                if closed.iter().contains(&position) {
                    #[cfg(test)]
                    println!("pos already closed {position:?}");
                    continue;
                }

                let g = current.g + 1;
                let h = position.get_distance(target);

                let next_node = Node { position, g, h };
                #[cfg(test)]
                println!("next_node {next_node:?}");

                if let Some(old_node) = open
                    .iter()
                    .find(|other| other.position == next_node.position)
                {
                    if old_node.g < g {
                        #[cfg(test)]
                        println!("old_node is already better");
                        continue;
                    }
                } else {
                    #[cfg(test)]
                    println!("pushing next_node into open");
                    open.push(next_node);
                    continue;
                }

                #[cfg(test)]
                println!("replacing position");
                open.retain(|x| x.position != next_node.position);
                open.push(next_node)
            }
        }
    }

    fn get_distance(&self, other: &Self) -> f64 {
        ((self.x.abs_diff(other.x).pow(2) + self.y.abs_diff(other.y).pow(2)) as f64).sqrt()
    }

    fn get_surrounding(&self) -> Vec<Self> {
        let mut surrounding = Vec::with_capacity(4);
        if self.x > 0 {
            surrounding.push(Self {
                x: self.x - 1,
                y: self.y,
            })
        }
        if self.y > 0 {
            surrounding.push(Self {
                x: self.x,
                y: self.y - 1,
            })
        }

        surrounding.push(Self {
            x: self.x + 1,
            y: self.y,
        });
        surrounding.push(Self {
            x: self.x,
            y: self.y + 1,
        });

        surrounding
    }
}

#[derive(Debug)]
struct Node {
    position: Position,
    g: usize,
    h: f64,
}

impl Node {
    fn f(&self) -> f64 {
        self.h + self.g as f64
    }
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position.eq(&other.position)
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.f().partial_cmp(&other.f()).map(|x| x.reverse())
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f().partial_cmp(&other.f()).unwrap().reverse()
    }
}

#[cfg(test)]
mod test {
    use crate::aoc_11_1::main;
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
    fn test() {
        assert_eq!(main(INPUT), 374);
    }
}
