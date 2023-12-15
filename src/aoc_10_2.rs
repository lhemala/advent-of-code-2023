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
use std::collections::hash_set::Union;
use std::collections::{HashMap, HashSet};
use std::fmt::{format, Display, Formatter, Write};
use std::fs::DirEntry;
use std::ops::{Not, Range};
use strum::{EnumIter, FromRepr, IntoEnumIterator};

#[allow(unused)]
pub fn aoc_10_2(input: &str) {
    println!("aoc_10_2: {}", main(input))
}

fn main(input: &str) -> i64 {
    let (_, mut maze) = separated_list1(line_ending, many1(Pipe::parse))(input).unwrap();

    let empty = Pipe {
        symbol: '.',
        ..Default::default()
    };

    for x in &mut maze {
        x.insert(0, empty.clone());
        x.push(empty.clone());
    }
    let width = maze.first().unwrap().len();
    maze.insert(0, vec![empty.clone(); width]);
    maze.push(vec![empty.clone(); width]);

    let start = maze
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .position(|pipe| pipe.symbol == 'S')
                .map(|x| Position { x, y })
        })
        .unwrap();

    #[cfg(test)]
    print_map(&maze);

    let mut positions = vec![];

    let mut current = start.clone();
    loop {
        get_at_pos_mut(&mut maze, current.x, current.y).visited = true;

        positions.push(current.clone());
        let next = current.get_next_pipe(&maze);
        if let Some(next) = next {
            current = next
        } else {
            break;
        }
    }

    let first = start.get_dir(positions.get(1).unwrap());
    let last = start.get_dir(positions.last().unwrap());
    let symbol = match (first, last) {
        (Direction::North, Direction::South) | (Direction::South, Direction::North) => '║',
        (Direction::North, Direction::West) | (Direction::West, Direction::North) => '╝',
        (Direction::North, Direction::East) | (Direction::East, Direction::North) => '╚',
        (Direction::South, Direction::West) | (Direction::West, Direction::South) => '╗',
        (Direction::South, Direction::East) | (Direction::East, Direction::South) => '╔',
        _ => panic!("unhandled"),
    };

    let pipe = get_at_pos_mut(&mut maze, start.x, start.y);
    pipe.visited = true;
    pipe.symbol = symbol;

    #[cfg(test)]
    print_map(&maze);

    let x_len = maze.first().unwrap().len();
    let y_len = maze.len();

    let points = (0..y_len)
        .flat_map(|y| (0..x_len).map(|x| (x, y)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    points.into_iter().fold(0, |acc, (x, y)| {
        if get_at_pos(&maze, x, y).visited {
            return acc;
        }

        let mut zz = maze.clone();
        let c = get_at_pos_mut(&mut zz, x, y);
        c.symbol = 'X';

        #[cfg(test)]
        print_map(&zz);

        let line = &maze.get(y).unwrap()[..x];
        let (pipe_count, _) = line.iter().fold((0, None), |(acc, last_up), pipe| {
            #[cfg(test)]
            println!("{} ({acc},{last_up:?})", pipe.symbol);

            if pipe.visited.not() {
                return (acc, None);
            }

            if pipe.symbol == '║' {
                return (acc + 1, None);
            }

            let end_up = ['╗', '╝']
                .into_iter()
                .find(|char| char == &pipe.symbol)
                .map(|x| x == '╝');
            match (last_up, end_up) {
                (None, None) => {
                    let start_up = ['╔', '╚']
                        .into_iter()
                        .find(|char| char == &pipe.symbol)
                        .map(|x| x == '╚');
                    (acc, start_up)
                }
                (Some(true), Some(true)) => (acc, None),
                (Some(true), Some(false)) | (Some(false), Some(true)) => (acc + 1, None),
                (Some(false), Some(false)) => (acc, None),
                (Some(_), None) => (acc, last_up),
                _ => panic!("unhandled"),
            }
        });
        #[cfg(test)]
        println!(
            "point ({x:>2},{y:>2}) pipe_count {pipe_count} -> {}",
            pipe_count % 2 != 0
        );
        if pipe_count % 2 != 0 {
            acc + 1
        } else {
            acc
        }
    })
}

fn print_map(maze: &[Vec<Pipe>]) {
    println!();
    maze.iter().for_each(|line| {
        println!(
            "{:?}",
            line.iter().map(|pipe| pipe.symbol.to_string()).join("")
        )
    })
}

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn get_next_pipe(&self, maze: &[Vec<Pipe>]) -> Option<Self> {
        let current = get_at_pos(maze, self.x, self.y);

        let north = get_at_pos(maze, self.x, self.y - 1);
        let west = get_at_pos(maze, self.x + 1, self.y);
        let south = get_at_pos(maze, self.x, self.y + 1);
        let east = get_at_pos(maze, self.x - 1, self.y);

        if current.connects(north, Direction::North) {
            Some(Self {
                x: self.x,
                y: self.y - 1,
            })
            //
        } else if current.connects(west, Direction::West) {
            Some(Self {
                x: self.x + 1,
                y: self.y,
            })
            //
        } else if current.connects(south, Direction::South) {
            Some(Self {
                x: self.x,
                y: self.y + 1,
            })
            //
        } else if current.connects(east, Direction::East) {
            Some(Self {
                x: self.x - 1,
                y: self.y,
            })
            //
        } else {
            None
        }
    }

    fn get_dir(&self, other: &Self) -> Direction {
        let x = self.x as i64 - other.x as i64;
        let y = self.y as i64 - other.y as i64;
        match (x, y) {
            (-1, 0) => Direction::East,
            (0, -1) => Direction::South,
            (1, 0) => Direction::West,
            (0, 1) => Direction::North,
            _ => panic!("unhandled"),
        }
    }
}

fn get_at_pos(maze: &[Vec<Pipe>], x: usize, y: usize) -> &Pipe {
    maze.get(y).unwrap().get(x).unwrap()
}

fn get_at_pos_mut(maze: &mut [Vec<Pipe>], x: usize, y: usize) -> &mut Pipe {
    maze.get_mut(y).unwrap().get_mut(x).unwrap()
}

#[derive(Debug, Default, Clone)]
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
                symbol: '║',
                north: true,
                south: true,
                ..Default::default()
            },
            '-' => Pipe {
                symbol: '═',
                east: true,
                west: true,
                ..Default::default()
            },
            'L' => Pipe {
                symbol: '╚',
                north: true,
                west: true,
                ..Default::default()
            },
            'J' => Pipe {
                symbol: '╝',
                north: true,
                east: true,
                ..Default::default()
            },
            '7' => Pipe {
                symbol: '╗',
                east: true,
                south: true,
                ..Default::default()
            },
            'F' => Pipe {
                symbol: '╔',
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
    use crate::aoc_10_2::main;
    use indoc::indoc;

    const INPUT_1: &str = indoc! {"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
        "
    };

    const INPUT_2: &str = indoc! {"
        ..........
        .S------7.
        .|F----7|.
        .||....||.
        .||....||.
        .|L-7F-J|.
        .|..||..|.
        .L--JL--J.
        ..........
        "
    };

    const INPUT_3: &str = indoc! {"
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
        "
    };

    const INPUT_4: &str = indoc! {"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
        "
    };

    #[test]
    fn test_1() {
        assert_eq!(main(INPUT_1), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(main(INPUT_2), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(main(INPUT_3), 8);
    }

    #[test]
    fn test_4() {
        assert_eq!(main(INPUT_4), 10);
    }
}
