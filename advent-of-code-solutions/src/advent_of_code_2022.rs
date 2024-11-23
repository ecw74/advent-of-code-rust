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
    solutions.insert(
        1,
        Box::new(aoc2022::Day01::new(1, YEAR, "Calorie Counting", day_01_src)),
    );

    // Add Day 2
    let day_02_src: &'static [u8] = include_bytes!("solutions/aoc2022/day02_impl.rs");
    solutions.insert(
        2,
        Box::new(aoc2022::Day02::new(
            2,
            YEAR,
            "Rock Paper Scissors",
            day_02_src,
        )),
    );

    // Add Day 3
    let day_03_src: &'static [u8] = include_bytes!("solutions/aoc2022/day03_impl.rs");
    solutions.insert(
        3,
        Box::new(aoc2022::Day03::new(
            3,
            YEAR,
            "Rucksack Reorganization",
            day_03_src,
        )),
    );

    // Add Day 4
    let day_04_src: &'static [u8] = include_bytes!("solutions/aoc2022/day04_impl.rs");
    solutions.insert(
        4,
        Box::new(aoc2022::Day04::new(4, YEAR, "Camp Cleanup", day_04_src)),
    );

    // Add Day 5
    let day_05_src: &'static [u8] = include_bytes!("solutions/aoc2022/day05_impl.rs");
    solutions.insert(
        5,
        Box::new(aoc2022::Day05::new(5, YEAR, "Supply Stacks", day_05_src)),
    );

    // Add Day 6
    let day_06_src: &'static [u8] = include_bytes!("solutions/aoc2022/day06_impl.rs");
    solutions.insert(
        6,
        Box::new(aoc2022::Day06::new(6, YEAR, "Tuning Trouble", day_06_src)),
    );

    Ok(())
}
