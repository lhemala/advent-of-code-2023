#![allow(unused_imports)]
use crate::aoc_01_1::aoc_01_1;
use crate::aoc_01_2::aoc_01_2;
use crate::aoc_02_1::aoc_02_1;
use crate::aoc_02_2::aoc_02_2;
use crate::aoc_03_1::aoc_03_1;
use crate::aoc_03_2::aoc_03_2;
use crate::aoc_04_1::aoc_04_1;
use crate::aoc_04_2::aoc_04_2;
use crate::aoc_05_1::aoc_05_1;
use crate::aoc_05_2::aoc_05_2;

mod aoc_01_1;
mod aoc_01_2;
mod aoc_02_1;
mod aoc_02_2;
mod aoc_03_1;
mod aoc_03_2;
mod aoc_04_1;
mod aoc_04_2;
mod aoc_05_1;
mod aoc_05_2;

fn main() {
    // let aoc_01_input = std::fs::read_to_string("input/01.txt").unwrap();
    // aoc_01_1(&aoc_01_input);
    // aoc_01_2(&aoc_01_input);

    // let aoc_02_input = std::fs::read_to_string("input/02.txt").unwrap();
    // aoc_02_1(&aoc_02_input);
    // aoc_02_2(&aoc_02_input);

    // let aoc_03_input = std::fs::read_to_string("input/03.txt").unwrap();
    // aoc_03_1(&aoc_03_input);
    // aoc_03_2(&aoc_03_input);

    // let aoc_04_input = std::fs::read_to_string("input/04.txt").unwrap();
    // aoc_04_1(&aoc_04_input);
    // aoc_04_2(&aoc_04_input);

    let aoc_05_input = std::fs::read_to_string("input/05.txt").unwrap();
    aoc_05_1(&aoc_05_input);
    aoc_05_2(&aoc_05_input);
}
