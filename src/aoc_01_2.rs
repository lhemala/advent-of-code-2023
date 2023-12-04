use logos::{Lexer, Logos};

#[allow(unused)]
pub fn aoc_01_2(input: &str) {
    println!("aoc_01_2: {}", main(input))
}

fn main(input: &str) -> i64 {
    input.lines().enumerate().map(get_number_of_line).sum()
}

fn get_number_of_line((i, input): (usize, &str)) -> i64 {
    let left_lex = LeftToken::lexer(input);
    let left_digits: Vec<i64> = left_lex
        .filter_map(|t| match t.unwrap() {
            LeftToken::Number(n) => Some(n),
            LeftToken::Digit(d) => Some(d),
            LeftToken::Text => None,
        })
        .collect();

    let rev_input = input.chars().rev().collect::<String>();
    let right_lex = RightToken::lexer(&rev_input);
    let right_digits: Vec<i64> = right_lex
        .filter_map(|t| match t.unwrap() {
            RightToken::Number(n) => Some(n),
            RightToken::Digit(d) => Some(d),
            RightToken::Text => None,
        })
        .collect();

    let num: i64 = left_digits.first().unwrap() * 10 + right_digits.first().unwrap();
    println!("{i}: |{input}| {left_digits:?} {right_digits:?} -> {num}");
    num
}

fn left_digit(lex: &mut Lexer<LeftToken>) -> Option<i64> {
    let slice = lex.slice();
    let n: i64 = slice[..slice.len()].parse().ok()?;
    Some(n)
}

fn right_digit(lex: &mut Lexer<RightToken>) -> Option<i64> {
    let slice = lex.slice();
    let n: i64 = slice[..slice.len()].parse().ok()?;
    Some(n)
}

fn left_number(lex: &mut Lexer<LeftToken>) -> Option<i64> {
    let n = match lex.slice() {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("fuck"),
    };
    Some(n)
}

fn right_number(lex: &mut Lexer<RightToken>) -> Option<i64> {
    let n = match lex.slice() {
        "orez" => 0,
        "eno" => 1,
        "owt" => 2,
        "eerht" => 3,
        "ruof" => 4,
        "evif" => 5,
        "xis" => 6,
        "neves" => 7,
        "thgie" => 8,
        "enin" => 9,
        _ => panic!("fuck |{}|", lex.slice()),
    };
    Some(n)
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[\n]+")] // Ignore this regex pattern between tokens
enum LeftToken {
    // #[token("zero", number, priority = 100)]
    #[token("one", left_number, priority = 100)]
    #[token("two", left_number, priority = 100)]
    #[token("three", left_number, priority = 100)]
    #[token("four", left_number, priority = 100)]
    #[token("five", left_number, priority = 100)]
    #[token("six", left_number, priority = 100)]
    #[token("seven", left_number, priority = 100)]
    #[token("eight", left_number, priority = 100)]
    #[token("nine", left_number, priority = 100)]
    Number(i64),

    #[regex(r"\d{1}", left_digit)]
    Digit(i64),

    #[regex(r"[a-zA-Z]{1}")]
    Text,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[\n]+")] // Ignore this regex pattern between tokens
enum RightToken {
    // #[token("orez", number, priority = 100)]
    #[token("eno", right_number, priority = 100)]
    #[token("owt", right_number, priority = 100)]
    #[token("eerht", right_number, priority = 100)]
    #[token("ruof", right_number, priority = 100)]
    #[token("evif", right_number, priority = 100)]
    #[token("xis", right_number, priority = 100)]
    #[token("neves", right_number, priority = 100)]
    #[token("thgie", right_number, priority = 100)]
    #[token("enin", right_number, priority = 100)]
    Number(i64),

    #[regex(r"\d{1}", right_digit)]
    Digit(i64),

    #[regex(r"[a-zA-Z]{1}")]
    Text,
}

#[cfg(test)]
mod test {
    use crate::aoc_01_2::main;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
    };

    #[test]
    fn test() {
        assert_eq!(main(INPUT), 281);
    }

    #[test]
    fn test2() {
        assert_eq!(main("sevenine"), 79)
    }

    #[test]
    fn test3() {
        assert_eq!(main("oneight"), 18)
    }
}
