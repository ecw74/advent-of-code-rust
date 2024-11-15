use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::Instant;

use advent_of_code_solutions::advent_of_code_2022;
use advent_of_code_solutions::advent_of_code_2023;
use advent_of_code_solutions::aoc_solution::AoCSolution;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "YEAR")]
    year: u32,
    #[arg(short, long, value_name = "DAY")]
    day: u32,
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let day = cli.day;
    let year = cli.year;
    let input = cli.input;

    let build_date = env!("BUILD_DATE");
    let commit_hash = env!("COMMIT_HASH_SHORT");

    println!("Build Date  : {}", build_date);
    println!("Commit Hash : {}", commit_hash);

    let mut f = File::open(input)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let mut solutions: BTreeMap<u32, BTreeMap<u32, Box<dyn AoCSolution>>> = BTreeMap::new();
    let mut solutions_2022: BTreeMap<u32, Box<dyn AoCSolution>> = BTreeMap::new();
    let mut solutions_2023: BTreeMap<u32, Box<dyn AoCSolution>> = BTreeMap::new();
    let _ = advent_of_code_2022(&mut solutions_2022);
    let _ = advent_of_code_2023(&mut solutions_2023);

    solutions.insert(2022, solutions_2022);
    solutions.insert(2023, solutions_2023);

    let aoc = solutions.get(&year).unwrap().get(&day).unwrap();

    println!("Run AoC Solution for day {:02} {}", day, year);

    let start_part_1 = Instant::now();
    let part_1 = aoc.part_1_final(buffer.as_str());
    let duration_part_1 = start_part_1.elapsed();
    println!("Part 1: {}, {:?}", part_1, duration_part_1);


    let start_part_2 = Instant::now();
    let part_2 = aoc.part_2_final(buffer.as_str());
    let duration_part_2 = start_part_2.elapsed();
    println!("Part 2: {}, {:?}", part_2, duration_part_2);

    Ok(())
}
