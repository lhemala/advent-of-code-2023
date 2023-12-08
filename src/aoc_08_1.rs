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
use std::ops::{ControlFlow, Range};
use strum::{EnumIter, FromRepr, IntoEnumIterator};

#[allow(unused)]
pub fn aoc_08_1(input: &str) {
    println!("aoc_08_1: {}", main(input))
}

fn main(input: &str) -> i64 {
    let (_, network) = Network::parse(input).unwrap();

    println!("{network:?}");

    let start = Node("AAA".to_string());
    let target = Node("ZZZ".to_string());

    let flow = network.instructions.iter().cycle().try_fold(
        (&start, 0usize),
        |(cur, acc), instruction| {
            if cur == &target {
                return ControlFlow::Break((cur, acc));
            }

            let nodes = network.nodes.get(cur).unwrap();
            let next = &nodes[instruction.clone() as usize];

            ControlFlow::Continue((next, acc + 1))
        },
    );

    match flow {
        ControlFlow::Continue((_, steps)) => steps as i64,
        ControlFlow::Break((_, steps)) => steps as i64,
    }
}

#[derive(Debug)]
struct Network {
    instructions: Vec<Instruction>,
    nodes: HashMap<Node, Vec<Node>>,
}

impl Network {
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
        let (input, node) = alpha1(input)?;
        let (input, _) = tag(" = (")(input)?;
        let (input, others) = separated_list1(tag(", "), alpha1)(input)?;
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
    use crate::aoc_08_1::main;
    use indoc::indoc;

    const INPUT_1: &str = indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
        "
    };

    const INPUT_2: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
        "
    };

    #[test]
    fn test_1() {
        assert_eq!(main(INPUT_1), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(main(INPUT_2), 6);
    }
}
