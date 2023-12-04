use crate::aoc_01_1::aoc_01_1;
use crate::aoc_01_2::aoc_01_2;

mod aoc_01_1;
mod aoc_01_2;

fn main() {
    let aoc_01_input = std::fs::read_to_string("input/01.txt").unwrap();
    aoc_01_1(&aoc_01_input);
    aoc_01_2(&aoc_01_input);
}
