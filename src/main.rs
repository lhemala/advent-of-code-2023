#![allow(unused_imports)]
use crate::aoc_01_1::aoc_01_1;
use crate::aoc_01_2::aoc_01_2;
use crate::aoc_02_1::aoc_02_1;
use crate::aoc_02_2::aoc_02_2;

mod aoc_01_1;
mod aoc_01_2;
mod aoc_02_1;
mod aoc_02_2;

fn main() {
    // let aoc_01_input = std::fs::read_to_string("input/01.txt").unwrap();
    // aoc_01_1(&aoc_01_input);
    // aoc_01_2(&aoc_01_input);

    let aoc_02_input = std::fs::read_to_string("input/02.txt").unwrap();
    aoc_02_1(&aoc_02_input);
    aoc_02_2(&aoc_02_input);
}
