use std::collections::BTreeMap;

use anyhow::Result;

use crate::aoc_solution::AoCSolution;
use crate::solutions::aoc2022;

const YEAR: u32 = 2022;

pub fn advent_of_code(solutions: &mut BTreeMap<u32, Box<dyn AoCSolution>>) -> Result<()> {
    // cleanup before file
    solutions.clear();

    // Add Day 1
    let day_01_src: &'static [u8] = include_bytes!("solutions/aoc2022/day01_impl.rs");
    solutions.insert(1, Box::new(aoc2022::Day01::new(1, YEAR, "Calorie Counting", day_01_src)));

    // Add Day 2
    let day_02_src: &'static [u8] = include_bytes!("solutions/aoc2022/day02_impl.rs");
    solutions.insert(2, Box::new(aoc2022::Day02::new(2, YEAR, "Rock Paper Scissors", day_02_src)));

    Ok(())
}