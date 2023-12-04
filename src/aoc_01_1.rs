use nom::character::complete::{alpha0, anychar};
use nom::combinator::map_res;
use nom::multi::many1;
use nom::IResult;

#[allow(unused)]
pub fn aoc_01_1(input: &str) {
    println!("aoc_01_1: {}", main(input))
}

fn main(input: &str) -> i64 {
    input.lines().enumerate().map(get_number_of_line).sum()
}

fn get_number_of_line((i, input): (usize, &str)) -> i64 {
    let (_, digits) = many1(extract_digit)(input).unwrap();
    let num: i64 = digits.first().unwrap() * 10 + digits.last().unwrap();
    println!("{i}: |{input}| {digits:?} -> {num}");
    num
}

fn extract_digit(input: &str) -> IResult<&str, i64> {
    let (input, _) = alpha0(input)?;
    map_res(anychar, |c| {
        Ok::<i64, anyhow::Error>(c.to_digit(10).unwrap() as i64)
    })(input)
}

#[cfg(test)]
mod test {
    use crate::aoc_01_1::main;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
    };

    #[test]
    fn test() {
        assert_eq!(main(INPUT), 142);
    }
}
