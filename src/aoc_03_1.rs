use std::cell::Cell;
use std::cmp::max;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

#[allow(unused)]
pub fn aoc_03_1(input: &str) {
    println!("aoc_03_1: {}", main(input))
}

fn main(input: &str) -> i64 {
    let map = input
        .lines()
        .rev()
        .map(|line| {
            let mut cols = line
                .chars()
                .map(|char| {
                    Rc::new(Cell::new(match char {
                        '.' => Token::Empty,
                        '0'..='9' => Token::Number(char.to_digit(10).unwrap() as i64),
                        symbol => Token::Symbol(symbol),
                    }))
                })
                .collect::<Vec<_>>();

            for i in 1..cols.len() {
                let current = cols.get(i).unwrap();
                let last = cols.get(i - 1).unwrap();
                if let (Token::Number(x), Token::Number(y)) = (
                    (*last.clone()).clone().into_inner(),
                    (*current.clone()).clone().into_inner(),
                ) {
                    last.set(Token::Number(x * 10 + y));
                    let x = last.clone();
                    let _ = std::mem::replace(&mut cols[i], x);
                }
            }

            cols
        })
        .collect::<Vec<_>>();

    let mut sum = 0i64;

    for y in 0..map.len() {
        let row = map.get(y).unwrap();
        for x in 0..row.len() {
            let cell = row.get(x).unwrap();
            if let Token::Symbol(_) = cell.get() {
                for y in (y - 1)..=(y + 1) {
                    if y > map.len() {
                        continue;
                    }

                    for x in (x - 1)..=(x + 1) {
                        if x > row.len() {
                            continue;
                        }

                        let other = map.get(y).unwrap().get(x).unwrap();
                        if let Token::Number(num) = other.get() {
                            sum += num;
                            other.set(Token::Empty)
                        }
                    }
                }
            }
        }
    }

    println!("{map:?}");

    sum
}

#[derive(Copy, Clone, Debug)]
enum Token {
    Number(i64),
    Symbol(char),
    Empty,
}

#[cfg(test)]
mod test {
    use crate::aoc_03_1::main;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."
    };

    #[test]
    fn test() {
        assert_eq!(main(INPUT), 4361);
    }
}
