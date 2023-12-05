use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha0, alpha1, anychar, digit1};
use nom::combinator::{complete, map_res, opt};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{pair, separated_pair};
use nom::IResult;
use std::cmp::max;
use std::collections::HashSet;
use std::fs::DirEntry;
use std::ops::Not;

#[allow(unused)]
pub fn aoc_04_2(input: &str) {
    println!("aoc_04_2: {}", main(input))
}

fn main(input: &str) -> i64 {
    let (_, cards) = separated_list1(tag("\n"), Card::parse)(input).unwrap();

    let cards: Vec<_> = cards
        .iter()
        .map(|card| (card.id, card.get_points()))
        .collect();

    let mut total_cards = cards.len();
    let mut queue = (0..cards.len()).collect::<Vec<_>>();
    while let Some(idx) = queue.pop() {
        let (id, points) = *cards.get(idx).unwrap();
        total_cards += points;
        queue.extend(id..(id + points));
    }

    total_cards as i64
}

#[derive(Debug)]
struct Card {
    #[allow(unused)]
    id: usize,
    winning: HashSet<i64>,
    numbers: Vec<i64>,
}

impl Card {
    fn parse(input: &str) -> IResult<&str, Card> {
        let (input, _) = tag("Card")(input)?;
        let (input, _) = many1(tag(" "))(input)?;
        let (input, id) = complete::i64(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, (winning, numbers)) =
            separated_pair(parse_numbers, tag("|"), parse_numbers)(input)?;

        let card = Card {
            id: id as usize,
            winning: HashSet::from_iter(winning),
            numbers,
        };
        println!("{card:?}");
        Ok((input, card))
    }

    fn get_points(&self) -> usize {
        self.numbers
            .iter()
            .filter(|&num| self.winning.contains(num))
            .count()
    }
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<i64>> {
    many1(parse_number)(input)
}

fn parse_number(input: &str) -> IResult<&str, i64> {
    let (input, _) = many0(tag(" "))(input)?;
    let (input, num) = complete::i64(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    Ok((input, num))
}

#[cfg(test)]
mod test {
    use crate::aoc_04_2::main;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    };

    #[test]
    fn test() {
        assert_eq!(main(INPUT), 30);
    }
}
