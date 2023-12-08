use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{
    alpha0, alpha1, alphanumeric1, anychar, char, digit1, line_ending, none_of, space1,
};
use nom::combinator::{complete, consumed, map_res, opt, recognize};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{pair, separated_pair};
use nom::IResult;
use rayon::iter::ParallelIterator;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::fs::DirEntry;
use std::ops::{ControlFlow, Range};
use strum::{EnumIter, FromRepr, IntoEnumIterator};

#[allow(unused)]
pub fn aoc_08_2(input: &str) {
    println!("aoc_08_2: {}", main(input))
}

fn main(input: &str) -> i64 {
    let (_, network) = Network::parse(input).unwrap();

    println!("{network:?}");

    let cycles = network
        .nodes
        .par_iter()
        .filter(|(node, _)| node.0.ends_with('A'))
        .map(|(start, _)| network.calc_cycle(start))
        .collect::<Vec<_>>();

    let cycles = cycles
        .into_iter()
        .map(|x| *x.first().unwrap())
        .collect::<Vec<_>>();

    lcm(&cycles)
}

#[allow(unused)]
fn main_naive(input: &str) -> i64 {
    let (_, network) = Network::parse(input).unwrap();

    println!("{network:?}");

    let start_nodes = network
        .nodes
        .keys()
        .filter(|node| node.0.ends_with('A'))
        .collect::<Vec<_>>();

    let flow = network.instructions.iter().cycle().try_fold(
        (start_nodes[0], 0usize, vec![]),
        |(cur, acc, mut ends), instruction| {
            if cur.0.ends_with('Z') {
                let tuple = (cur, instruction.clone(), acc);
                if ends.contains(&tuple) {
                    return ControlFlow::Break(
                        ends.into_iter()
                            .map(|(_, _, acc)| acc as i64)
                            .collect::<Vec<i64>>(),
                    );
                }
                ends.push(tuple);
                return ControlFlow::Continue((cur, acc, ends));
            }

            let nodes = network.nodes.get(cur).unwrap();
            let next = &nodes[instruction.clone() as usize];

            ControlFlow::Continue((next, acc + 1, ends))
        },
    );

    match flow {
        ControlFlow::Break(_) => 0,
        _ => panic!("unhandled"),
    }
}

#[derive(Debug)]
struct Network {
    instructions: Vec<Instruction>,
    nodes: HashMap<Node, Vec<Node>>,
}

impl Network {
    fn calc_cycle(&self, start: &Node) -> Vec<i64> {
        let flow = self.instructions.iter().cycle().try_fold(
            (start, 0usize, vec![]),
            |(cur, acc, mut ends), instruction| {
                if cur.0.ends_with('Z') {
                    let tuple = (cur, instruction.clone(), acc);
                    if ends.contains(&tuple) {
                        println!("cycle: {ends:?}");
                        return ControlFlow::Break(
                            ends.into_iter()
                                .map(|(_, _, acc)| acc as i64)
                                .collect::<Vec<i64>>(),
                        );
                    }
                    ends.push(tuple);
                    return ControlFlow::Continue((cur, acc, ends));
                }

                let nodes = self.nodes.get(cur).unwrap();
                let next = &nodes[instruction.clone() as usize];

                ControlFlow::Continue((next, acc + 1, ends))
            },
        );

        match flow {
            ControlFlow::Break(x) => x,
            _ => panic!("unhandled"),
        }
    }
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, instructions) = many1(Instruction::parse)(input)?;

        let (input, _) = many1(line_ending)(input)?;

        let (input, nodes) = many1(Self::parse_node)(input)?;

        Ok((
            input,
            Self {
                instructions,
                nodes: HashMap::from_iter(nodes),
            },
        ))
    }

    fn parse_node(input: &str) -> IResult<&str, (Node, Vec<Node>)> {
        let (input, node) = alphanumeric1(input)?;
        let (input, _) = tag(" = (")(input)?;
        let (input, others) = separated_list1(tag(", "), alphanumeric1)(input)?;
        let (input, _) = tag(")")(input)?;
        let (input, _) = opt(line_ending)(input)?;

        Ok((
            input,
            (
                Node(node.to_string()),
                others
                    .into_iter()
                    .map(|str| Node(str.to_string()))
                    .collect(),
            ),
        ))
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum Instruction {
    Left = 0,
    Right = 1,
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, char) = none_of(" \r\n")(input)?;

        let instruction = match char {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            other => panic!("unhandled '{other}'"),
        };

        Ok((input, instruction))
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Node(String);

#[cfg(test)]
mod test {
    use crate::aoc_08_2::main;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
        "
    };

    #[test]
    fn test_1() {
        assert_eq!(main(INPUT), 6);
    }
}

pub fn lcm(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
