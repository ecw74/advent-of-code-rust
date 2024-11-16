use std::collections::BTreeMap;

use anyhow::Result;
use aoc_solution::AoCSolution;

pub mod aoc_solution;

pub mod solutions;

mod advent_of_code_2022;

pub fn advent_of_code_2022(solutions: &mut BTreeMap<u32, Box<dyn AoCSolution>>) -> Result<()> {
    crate::advent_of_code_2022::advent_of_code(solutions)
}

mod advent_of_code_2023;

pub fn advent_of_code_2023(solutions: &mut BTreeMap<u32, Box<dyn AoCSolution>>) -> Result<()> {
    crate::advent_of_code_2023::advent_of_code(solutions)
}

mod advent_of_code_2024;

pub fn advent_of_code_2024(solutions: &mut BTreeMap<u32, Box<dyn AoCSolution>>) -> Result<()> {
    crate::advent_of_code_2024::advent_of_code(solutions)
}
