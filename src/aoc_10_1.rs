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
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::fs::DirEntry;
use std::ops::{Not, Range};
use strum::{EnumIter, FromRepr, IntoEnumIterator};

#[allow(unused)]
pub fn aoc_10_1(input: &str) {
    println!("aoc_10_1: {}", main(input))
}

fn main(input: &str) -> i64 {
    let (_, mut maze) = separated_list1(line_ending, many1(Pipe::parse))(input).unwrap();

    #[cfg(test)]
    maze.iter().for_each(|line| {
        println!(
            "{:?}",
            line.iter().map(|pipe| pipe.symbol).collect::<Vec<_>>()
        )
    });

    let start = maze
        .iter()
        .enumerate()
        .find_map(|(x, line)| {
            line.iter()
                .position(|pipe| pipe.symbol == 'S')
                .map(|y| Position { x, y })
        })
        .unwrap();

    #[cfg(test)]
    println!(
        "start @ {start:?} -> {:?}",
        get_at_pos(&maze, start.x as i64, start.y as i64)
    );

    let mut current = start.clone();
    let mut steps = 0;
    while steps == 0 || current != start {
        if steps > 0 {
            get_at_pos_mut(&mut maze, current.x, current.y).visited = true;
        }

        steps += 1;
        current = current.get_next(&maze);

        #[cfg(test)]
        println!("current: {current:?}")
    }

    steps / 2
}

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn get_next(self, maze: &Vec<Vec<Pipe>>) -> Self {
        let current = get_at_pos(maze, self.x as i64, self.y as i64);

        let north = get_at_pos(maze, self.x as i64, self.y as i64 - 1);
        let west = get_at_pos(maze, self.x as i64 + 1, self.y as i64);
        let south = get_at_pos(maze, self.x as i64, self.y as i64 + 1);
        let east = get_at_pos(maze, self.x as i64 - 1, self.y as i64);

        if current.connects(north, Direction::North) {
            #[cfg(test)]
            println!(
                "moving north to {:?}",
                get_at_pos(maze, self.x as i64, self.y as i64 - 1)
            );
            Self {
                x: self.x,
                y: self.y - 1,
            }
            //
        } else if current.connects(west, Direction::West) {
            #[cfg(test)]
            println!(
                "moving west to {:?}",
                get_at_pos(maze, self.x as i64 + 1, self.y as i64)
            );
            Self {
                x: self.x + 1,
                y: self.y,
            }
            //
        } else if current.connects(south, Direction::South) {
            #[cfg(test)]
            println!(
                "moving south to {:?}",
                get_at_pos(maze, self.x as i64, self.y as i64 + 1)
            );
            Self {
                x: self.x,
                y: self.y + 1,
            }
            //
        } else if current.connects(east, Direction::East) {
            #[cfg(test)]
            println!(
                "moving east to {:?}",
                get_at_pos(maze, self.x as i64 - 1, self.y as i64)
            );
            Self {
                x: self.x - 1,
                y: self.y,
            }
            //
        } else {
            panic!("unhandled")
        }
    }
}

fn get_at_pos(maze: &Vec<Vec<Pipe>>, x: i64, y: i64) -> &Pipe {
    let x = if x < 0 {
        0usize
    } else if x == maze.first().unwrap().len() as i64 {
        maze.first().unwrap().len() - 1
    } else {
        x as usize
    };
    let y = if y < 0 {
        0usize
    } else if y == maze.len() as i64 {
        maze.first().unwrap().len() - 1
    } else {
        y as usize
    };

    maze.get(y).unwrap().get(x).unwrap()
}

fn get_at_pos_mut(maze: &mut [Vec<Pipe>], x: usize, y: usize) -> &mut Pipe {
    maze.get_mut(y).unwrap().get_mut(x).unwrap()
}

#[derive(Debug, Default)]
struct Pipe {
    visited: bool,
    symbol: char,
    north: bool,
    west: bool,
    south: bool,
    east: bool,
}

impl Pipe {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, char) = none_of(" \r\n")(input)?;

        let pipe = match char {
            '|' => Pipe {
                symbol: '|',
                north: true,
                south: true,
                ..Default::default()
            },
            '-' => Pipe {
                symbol: '-',
                east: true,
                west: true,
                ..Default::default()
            },
            'L' => Pipe {
                symbol: 'L',
                north: true,
                west: true,
                ..Default::default()
            },
            'J' => Pipe {
                symbol: 'J',
                north: true,
                east: true,
                ..Default::default()
            },
            '7' => Pipe {
                symbol: '7',
                east: true,
                south: true,
                ..Default::default()
            },
            'F' => Pipe {
                symbol: 'F',
                west: true,
                south: true,
                ..Default::default()
            },
            '.' => Pipe {
                symbol: '.',
                ..Default::default()
            },
            'S' => Pipe {
                symbol: 'S',
                north: true,
                east: true,
                south: true,
                west: true,
                ..Default::default()
            },
            _ => panic!("unhandled"),
        };

        Ok((input, pipe))
    }

    fn connects(&self, other: &Pipe, dir: Direction) -> bool {
        match dir {
            Direction::North => other.visited.not() && self.north && other.south,
            Direction::East => other.visited.not() && self.east && other.west,
            Direction::South => other.visited.not() && self.south && other.north,
            Direction::West => other.visited.not() && self.west && other.east,
        }
    }
}

enum Direction {
    North = 1,
    East = 2,
    South = 3,
    West = 4,
}

#[cfg(test)]
mod test {
    use crate::aoc_10_1::main;
    use indoc::indoc;

    const INPUT_1: &str = indoc! {"
        -L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF
        "
    };

    const INPUT_2: &str = indoc! {"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
        "
    };

    #[test]
    fn test_1() {
        assert_eq!(main(INPUT_1), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(main(INPUT_2), 8);
    }
}
