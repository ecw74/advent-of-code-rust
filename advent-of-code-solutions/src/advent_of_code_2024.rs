use std::collections::BTreeMap;

use anyhow::Result;
use solutions::aoc2024;

use crate::aoc_solution::AoCSolution;
use crate::solutions;

const YEAR: u32 = 2024;

pub fn advent_of_code(solutions: &mut BTreeMap<u32, Box<dyn AoCSolution>>) -> Result<()> {
    // cleanup before file
    solutions.clear();

    // Add Day 1
    let day_01_src: &'static [u8] = include_bytes!("solutions/aoc2024/day01_impl.rs");
    solutions.insert(1, Box::new(aoc2024::Day01::new(1, YEAR, "Foo", day_01_src)));

    Ok(())
}