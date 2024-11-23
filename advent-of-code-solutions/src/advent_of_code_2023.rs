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
    solutions.insert(
        1,
        Box::new(aoc2023::Day01::new(1, YEAR, "Trebuchet?!", day_01_src)),
    );

    // Add Day 2
    let day_02_src: &'static [u8] = include_bytes!("solutions/aoc2023/day02_impl.rs");
    solutions.insert(
        2,
        Box::new(aoc2023::Day02::new(2, YEAR, "Cube Conundrum", day_02_src)),
    );

    // Add Day 3
    let day_03_src: &'static [u8] = include_bytes!("solutions/aoc2023/day03_impl.rs");
    solutions.insert(
        3,
        Box::new(aoc2023::Day03::new(3, YEAR, "Gear Ratios", day_03_src)),
    );

    // Add Day 4
    let day_04_src: &'static [u8] = include_bytes!("solutions/aoc2023/day04_impl.rs");
    solutions.insert(
        4,
        Box::new(aoc2023::Day04::new(4, YEAR, "Scratchcards", day_04_src)),
    );

    // Add Day 5
    let day_05_src: &'static [u8] = include_bytes!("solutions/aoc2023/day05_impl.rs");
    solutions.insert(
        5,
        Box::new(aoc2023::Day05::new(
            5,
            YEAR,
            "If You Give A Seed A Fertilizer",
            day_05_src,
        )),
    );

    Ok(())
}
