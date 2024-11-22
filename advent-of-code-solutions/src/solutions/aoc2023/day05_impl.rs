use super::day05::Day05;

#[derive(Debug)]
/// Represents a mapping entry between source and target ranges.
struct MapEntry {
    source_begin: u64, // Start of the source range
    source_end: u64,   // End of the source range
    target_begin: u64, // Start of the target range
}

impl Day05 {
    /// Parses the puzzle input into:
    /// 1. A vector of seed numbers (or ranges for part 2).
    /// 2. A vector of maps, where each map is a vector of `MapEntry`.
    fn parse_input(input: &str) -> (Vec<u64>, Vec<Vec<MapEntry>>) {
        let lines = input.lines();
        let mut maps = Vec::with_capacity(10); // Allocate space for up to 10 maps
        let mut current_entries = Vec::with_capacity(50); // Allocate space for up to 50 entries per map

        // Parse the seed numbers from the `seeds:` line.
        let seeds = input
            .lines()
            .filter(|line| !line.trim().is_empty()) // Skip empty lines
            .filter_map(|line| {
                if line.starts_with("seeds:") {
                    Some(
                        line["seeds:".len()..] // Extract everything after "seeds:"
                            .split_whitespace()
                            .filter_map(|s| s.parse::<u64>().ok()) // Parse as `u64`
                            .collect::<Vec<u64>>(),
                    )
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        // Parse the maps from the input.
        for line in lines {
            if line.contains("map:") {
                // A new map starts; save the current map (if any).
                if !current_entries.is_empty() {
                    maps.push(current_entries);
                    current_entries = Vec::with_capacity(50); // Reinitialize for the next map
                }
            } else if !line.trim().is_empty() {
                // Parse individual map entries
                let parts: Vec<u64> = line
                    .split_whitespace()
                    .filter_map(|s| s.parse::<u64>().ok())
                    .collect();

                if parts.len() == 3 {
                    let source_begin = parts[1];
                    let source_end = parts[1] + parts[2]; // Compute source range end
                    let target_begin = parts[0];

                    let entry = MapEntry {
                        source_begin,
                        source_end,
                        target_begin,
                    };

                    current_entries.push(entry);
                }
            }
        }

        // Add the last map (if any).
        if !current_entries.is_empty() {
            maps.push(current_entries);
        }

        (seeds, maps)
    }

    /// Maps a source range to a target range based on the given map entries.
    /// Returns a vector of resulting ranges.
    fn map_source_to_target(range: (u64, u64), mapping: &[MapEntry]) -> Vec<(u64, u64)> {
        let (start, end) = range;
        let mut result = Vec::new();

        for entry in mapping {
            let source_end = entry.source_begin + (entry.source_end - entry.source_begin) - 1;
            // Check if the source range overlaps with the current entry's range.
            if entry.source_begin <= end && start <= source_end {
                let mapped_start = start.max(entry.source_begin);
                let mapped_end = end.min(source_end);
                let offset = mapped_start - entry.source_begin;

                result.push((
                    entry.target_begin + offset,
                    entry.target_begin + offset + (mapped_end - mapped_start),
                ));
            }
        }

        // If no overlap was found, the range maps to itself.
        if result.is_empty() {
            result.push((start, end));
        }

        result
    }

    /// Merges overlapping or contiguous ranges into a single range.
    fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
        if ranges.is_empty() {
            return vec![];
        }

        // Sort ranges by their starting value.
        ranges.sort_by(|a, b| a.0.cmp(&b.0));

        let mut merged = Vec::new();
        merged.push(ranges[0]);

        for range in ranges.iter().skip(1) {
            let last = merged.last_mut().unwrap();
            if range.0 <= last.1 + 1 {
                // Extend the last range if they overlap or are contiguous.
                last.1 = last.1.max(range.1);
            } else {
                merged.push(*range);
            }
        }

        merged
    }

    /// Solves part 1 of the puzzle: finds the smallest location for individual seeds.
    pub fn part_1(&self, _input: &str) -> String {
        let (seeds, maps) = Self::parse_input(_input);

        // Convert seeds into ranges where each range is a single number.
        let mut current_ranges: Vec<(u64, u64)> =
            seeds.iter().map(|&start| (start, start)).collect();

        for map in &maps {
            let mut next_ranges = Vec::new();

            // Map each range through the current map.
            for range in &current_ranges {
                let mapped_ranges = Self::map_source_to_target(*range, map);
                next_ranges.extend(mapped_ranges);
            }

            // Merge overlapping ranges.
            current_ranges = Self::merge_ranges(next_ranges);
        }

        // Find the smallest starting value in the resulting ranges.
        current_ranges
            .iter()
            .min_by_key(|&&(start, _)| start)
            .map(|&(start, _)| start)
            .unwrap_or_default()
            .to_string()
    }

    /// Solves part 2 of the puzzle: handles ranges of seeds.
    pub fn part_2(&self, _input: &str) -> String {
        let (seeds, maps) = Self::parse_input(_input);

        // Parse seeds as ranges (start, end).
        let mut current_ranges: Vec<(u64, u64)> = seeds
            .chunks(2) // Each range is defined by two values: start and length.
            .map(|chunk| (chunk[0], chunk[0] + chunk[1] - 2)) // Compute end from start + length.
            .collect();

        for map in &maps {
            let mut next_ranges = Vec::new();

            // Map each range through the current map.
            for range in &current_ranges {
                let mapped_ranges = Self::map_source_to_target(*range, map);
                next_ranges.extend(mapped_ranges);
            }

            // Merge overlapping ranges.
            current_ranges = Self::merge_ranges(next_ranges);
        }

        // Find the smallest starting value in the resulting ranges.
        current_ranges
            .iter()
            .min_by_key(|&&(start, _)| start)
            .map(|&(start, _)| start)
            .unwrap_or_default()
            .to_string()
    }
}

mod test {
    #[test]
    /// Test for part 1 of Day05 for AoC 2023.
    /// Ensures correct calculation of the sum of first and last digits in each line.
    fn test_aoc2023_day05_part_1() {
        use crate::solutions::aoc2023::Day05;
        let day05 = Day05 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        "#;
        assert_eq!(day05.part_1(input), "35"); // Asserts if the function output matches the expected result.
    }

    #[test]
    /// Test for part 2 of Day05 for AoC 2023.
    /// Verifies correct handling and sum of lines with spelled-out numbers.
    fn test_aoc2023_day05_part_2() {
        use crate::solutions::aoc2023::Day05;
        let day05 = Day05 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        "#;
        assert_eq!(day05.part_2(input), "46"); // Asserts if the function output matches the expected result.
    }
}
