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
    solutions.insert(
        1,
        Box::new(aoc2024::Day01::new(
            1,
            YEAR,
            "Historian Hysteria",
            day_01_src,
        )),
    );

    // Add Day 2
    let day_02_src: &'static [u8] = include_bytes!("solutions/aoc2024/day02_impl.rs");
    solutions.insert(
        2,
        Box::new(aoc2024::Day02::new(
            2,
            YEAR,
            "Red-Nosed Reports",
            day_02_src,
        )),
    );

    // Add Day 3
    let day_03_src: &'static [u8] = include_bytes!("solutions/aoc2024/day03_impl.rs");
    solutions.insert(
        3,
        Box::new(aoc2024::Day03::new(3, YEAR, "Mull It Over", day_03_src)),
    );

    // Add Day 4
    let day_04_src: &'static [u8] = include_bytes!("solutions/aoc2024/day04_impl.rs");
    solutions.insert(
        4,
        Box::new(aoc2024::Day04::new(4, YEAR, "Ceres Search", day_04_src)),
    );

    // Add Day 5
    let day_05_src: &'static [u8] = include_bytes!("solutions/aoc2024/day05_impl.rs");
    solutions.insert(
        5,
        Box::new(aoc2024::Day05::new(5, YEAR, "Print Queue", day_05_src)),
    );

    // Add Day 6
    let day_06_src: &'static [u8] = include_bytes!("solutions/aoc2024/day06_impl.rs");
    solutions.insert(
        6,
        Box::new(aoc2024::Day06::new(6, YEAR, "Guard Gallivant", day_06_src)),
    );

    // Add Day 7
    let day_07_src: &'static [u8] = include_bytes!("solutions/aoc2024/day07_impl.rs");
    solutions.insert(
        7,
        Box::new(aoc2024::Day07::new(7, YEAR, "Bridge Repair", day_07_src)),
    );

    // Add Day 8
    let day_08_src: &'static [u8] = include_bytes!("solutions/aoc2024/day08_impl.rs");
    solutions.insert(
        8,
        Box::new(aoc2024::Day08::new(
            8,
            YEAR,
            "Resonant Collinearity",
            day_08_src,
        )),
    );

    Ok(())
}
