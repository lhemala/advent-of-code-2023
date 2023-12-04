use nom::bytes::complete::tag;
use nom::character::complete::{alpha0, alpha1, anychar, digit1, i64};
use nom::combinator::{map_res, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::pair;
use nom::IResult;

#[allow(unused)]
pub fn aoc_02_1(input: &str) {
    println!("aoc_02_1: {}", main(input, 12, 13, 14))
}

fn main(input: &str, red: i64, green: i64, blue: i64) -> i64 {
    let (_, games) = separated_list1(tag("\n"), Game::parse)(input).unwrap();

    games
        .iter()
        .filter_map(|game| {
            if game.is_possible(red, green, blue) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

#[derive(Debug)]
struct Game {
    id: i64,
    sets: Vec<Set>,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Game> {
        let (input, _) = tag("Game ")(input)?;
        let (input, id) = i64(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, sets) = separated_list1(tag(";"), Set::parse)(input)?;

        let game = Game { id, sets };
        println!("{game:?}");
        Ok((input, game))
    }

    fn is_possible(&self, red: i64, green: i64, blue: i64) -> bool {
        self.sets
            .iter()
            .all(|set| set.red <= red && set.blue <= blue && set.green <= green)
    }
}

#[derive(Debug, Default)]
struct Set {
    red: i64,
    green: i64,
    blue: i64,
}

impl Set {
    fn parse(input: &str) -> IResult<&str, Set> {
        let (input, colors) = many1(Color::parse)(input)?;

        let set = colors.iter().fold(Set::default(), |mut set, color| {
            match color {
                Color::Red(count) => set.red = *count,
                Color::Green(count) => set.green = *count,
                Color::Blue(count) => set.blue = *count,
            };
            set
        });

        Ok((input, set))
    }
}

enum Color {
    Red(i64),
    Green(i64),
    Blue(i64),
}

impl Color {
    fn parse(input: &str) -> IResult<&str, Color> {
        let (input, _) = tag(" ")(input)?;
        let (input, count) = i64(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, raw_color) = alpha1(input)?;
        let (input, _) = opt(tag(","))(input)?;

        let color = match raw_color {
            "red" => Color::Red(count),
            "blue" => Color::Blue(count),
            "green" => Color::Green(count),
            other => panic!("fuck {other}"),
        };

        Ok((input, color))
    }
}

#[cfg(test)]
mod test {
    use crate::aoc_02_1::main;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    };

    #[test]
    fn test() {
        assert_eq!(main(INPUT, 12, 13, 14), 8);
    }
}
