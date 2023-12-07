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
pub fn aoc_07_1(input: &str) {
    println!("aoc_07_1: {}", main(input))
}

fn main(input: &str) -> i64 {
    let (_, mut hands) = many1(Hand::parse)(input).unwrap();

    hands.sort_by(|hand, other| hand.strength.cmp(&other.strength));

    println!(
        "{}",
        hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| format!("{:>4} {:>4} {:?}", rank + 1, hand.bet, hand))
            .collect::<Vec<_>>()
            .join("\n")
    );

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, hand)| acc + (rank + 1) as i64 * hand.bet)
}

#[derive(Debug)]
struct Hand {
    #[allow(unused)] // only used in debug log
    cards: Vec<Card>,
    #[allow(unused)] // only used in debug log
    hand_type: Type,
    strength: Vec<i64>,
    bet: i64,
}

impl Hand {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, hand_cards) = many1(Card::parse)(input)?;
        let (input, _) = space1(input)?;
        let (input, bet) = complete::i64(input)?;
        let (input, _) = opt(line_ending)(input)?;

        let cards: HashMap<Card, usize> =
            hand_cards.iter().fold(HashMap::new(), |mut acc, card| {
                let count = match acc.get(card) {
                    Some(count) => count + 1,
                    None => 1,
                };
                acc.insert(card.clone(), count);
                acc
            });

        let hand_type = Type::from(cards);

        let mut strength = vec![hand_type.clone() as i64];
        strength.extend(hand_cards.iter().map(|c| c.clone() as i64));

        Ok((
            input,
            Self {
                cards: hand_cards,
                hand_type,
                bet,
                strength,
            },
        ))
    }
}

#[derive(Debug, FromRepr, Eq, Hash, PartialEq, Clone)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl Card {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, c) = none_of(" ")(input)?;

        let card = match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            other => Self::from_repr(other.to_digit(10).unwrap() as usize).unwrap(),
        };

        Ok((input, card))
    }
}

#[derive(Debug, Clone)]
enum Type {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl From<HashMap<Card, usize>> for Type {
    fn from(input: HashMap<Card, usize>) -> Self {
        if Self::has_kind(&input, 5) {
            Self::FiveOfAKind
        } else if Self::has_kind(&input, 4) {
            Self::FourOfAKind
        } else if Self::has_kind(&input, 3) && Self::has_kind(&input, 2) {
            Self::FullHouse
        } else if Self::has_kind(&input, 3) {
            Self::ThreeOfAKind
        } else if Self::get_count_of_kind(&input, 2) == 2 {
            Self::TwoPair
        } else if Self::has_kind(&input, 2) {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

impl Type {
    fn get_count_of_kind(input: &HashMap<Card, usize>, kind: usize) -> usize {
        input.values().filter(|&count| count == &kind).count()
    }

    fn has_kind(input: &HashMap<Card, usize>, kind: usize) -> bool {
        input.values().any(|count| count == &kind)
    }
}

#[cfg(test)]
mod test {
    use crate::aoc_07_1::main;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"
    };

    #[test]
    fn test() {
        assert_eq!(main(INPUT), 6440);
    }
}
