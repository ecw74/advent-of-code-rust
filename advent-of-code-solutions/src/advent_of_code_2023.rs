use std::collections::BTreeMap;

use anyhow::Result;
use solutions::aoc2023;

use crate::aoc_solution::AoCSolution;
use crate::solutions;

const YEAR: u32 = 2023;

pub fn advent_of_code(solutions: &mut BTreeMap<u32, Box<dyn AoCSolution>>) -> Result<()> {
    // cleanup before file
    solutions.clear();

    // Add Day 1
    let day_01_src: &'static [u8] = include_bytes!("solutions/aoc2023/day01_impl.rs");
    solutions.insert(1, Box::new(aoc2023::Day01::new(1, YEAR, "Trebuchet?!", day_01_src)));

    // Add Day 2
    let day_02_src: &'static [u8] = include_bytes!("solutions/aoc2023/day02_impl.rs");
    solutions.insert(2, Box::new(aoc2023::Day02::new(2, YEAR, "Cube Conundrum", day_02_src)));

    Ok(())
}