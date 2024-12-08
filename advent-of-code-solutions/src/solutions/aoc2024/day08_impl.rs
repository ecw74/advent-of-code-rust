use super::day08::Day08;
use std::collections::{HashMap, HashSet};

impl Day08 {
    /// Parses a 2D grid string into a `HashMap` where keys are the antenna frequencies (characters),
    /// and values are `Vec` of (x, y) coordinates representing the locations of those antennas.
    /// The function also computes and returns the width and height of the grid.
    pub fn parse_antenna_grid(grid: &str) -> (i8, i8, HashMap<char, Vec<(i8, i8)>>) {
        let mut result: HashMap<char, Vec<(i8, i8)>> = HashMap::new();

        // Width and height are assumed to be the same and based on the number of non-empty lines.
        let width = grid.lines().filter(|s| !s.trim().is_empty()).count() as i8;
        let height = width;

        // Process only the non-empty lines, trimming whitespace.
        let lines = grid
            .lines()
            .filter(|s| !s.trim().is_empty())
            .map(|line| line.trim());

        // Iterate through each line (y-coordinate) and each character in the line (x-coordinate).
        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                // Skip empty cells represented by `.`.
                if c != '.' {
                    // Add the (x, y) location of the antenna to the hashmap under its frequency key.
                    result
                        .entry(c)
                        .or_insert_with(Vec::new)
                        .push((x as i8, y as i8));
                }
            }
        }

        (width, height, result)
    }

    /// Computes and inserts antinodes for a pair of antennas based on the Part 1 resonance rule.
    /// An antinode is valid if one antenna is twice as far as the other from the antinode position.
    fn find_antinodes(
        height: i8,
        width: i8,
        loc_0: (i8, i8),
        loc_1: (i8, i8),
        antinodes: &mut HashSet<(i8, i8)>,
    ) {
        // For each antenna pair, test both combinations: loc_0 closer and loc_1 farther, and vice versa.
        for &(r0, c0, r1, c1) in &[
            (loc_0.0, loc_0.1, loc_1.0, loc_1.1),
            (loc_1.0, loc_1.1, loc_0.0, loc_0.1),
        ] {
            // Calculate the antinode position using the formula: ar = 2 * r0 - r1, ac = 2 * c0 - c1.
            let ar = 2 * r0 - r1;
            let ac = 2 * c0 - c1;

            // Ensure the computed antinode position is within grid bounds.
            if ar >= 0 && ar < height && ac >= 0 && ac < width {
                antinodes.insert((ar, ac)); // Add the valid antinode to the set.
            }
        }
    }

    /// Solves Part 1 of the puzzle: Counts unique antinode positions based on Part 1 resonance rules.
    pub fn part_1(&self, input: &str) -> String {
        // Parse the grid into dimensions and antenna locations.
        let (height, width, antenna_locations) = Self::parse_antenna_grid(input);
        let mut antinodes = HashSet::new(); // Set to store unique antinode positions.

        // Process each frequency group (antennas with the same character).
        antenna_locations.into_values().for_each(|locations| {
            locations
                .iter()
                .enumerate()
                .flat_map(|(i, &loc_0)| {
                    locations
                        .iter()
                        .skip(i + 1)
                        .map(move |&loc_1| (loc_0, loc_1))
                })
                .for_each(|(loc_0, loc_1)| {
                    Self::find_antinodes(height, width, loc_0, loc_1, &mut antinodes);
                });
        });

        // Return the count of unique antinodes as a string.
        antinodes.len().to_string()
    }

    /// Computes antinodes based on the Part 2 resonance rule, which considers collinear positions.
    fn compute_antinodes(
        height: i8,
        width: i8,
        loc_0: (i8, i8),
        loc_1: (i8, i8),
        antinodes: &mut HashSet<(i8, i8)>,
    ) {
        // Calculate the directional vector (dr, dc) between the two antennas.
        let mut dr = loc_1.0 - loc_0.0;
        let mut dc = loc_1.1 - loc_0.1;

        // Reduce the direction vector to its simplest form using the greatest common divisor (GCD).
        let gcd = Self::gcd(dr.abs(), dc.abs());
        dr /= gcd;
        dc /= gcd;

        // Explore all positions along the vector in both directions (forward and backward).
        for &dir in &[1, -1] {
            let (mut r, mut c) = loc_0;

            // Step in the direction until moving out of grid bounds.
            while r >= 0 && r < height && c >= 0 && c < width {
                antinodes.insert((r, c)); // Add each valid position to the set.
                r += dir * dr;
                c += dir * dc;
            }
        }
    }

    /// Helper function to compute the greatest common divisor (GCD) using recursion.
    fn gcd(a: i8, b: i8) -> i8 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    /// Solves Part 2 of the puzzle: Counts unique antinode positions based on the updated rules.
    pub fn part_2(&self, input: &str) -> String {
        // Parse the grid into dimensions and antenna locations.
        let (height, width, antenna_locations) = Self::parse_antenna_grid(input);
        let mut antinodes = HashSet::new(); // Set to store unique antinode positions.

        // Process each frequency group (antennas with the same character).
        antenna_locations.into_values().for_each(|locations| {
            locations
                .iter()
                .enumerate()
                .flat_map(|(i, &loc_0)| {
                    locations
                        .iter()
                        .skip(i + 1)
                        .map(move |&loc_1| (loc_0, loc_1))
                })
                .for_each(|(loc_0, loc_1)| {
                    Self::compute_antinodes(height, width, loc_0, loc_1, &mut antinodes);
                });
        });

        // Return the count of unique antinodes as a string.
        antinodes.len().to_string()
    }
}

mod test {
    /// Test for Part 1 of Day08.
    #[test]
    fn test_aoc2024_day08_part_1() {
        use crate::solutions::aoc2024::Day08;
        let day08 = Day08 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
        "#;
        assert_eq!(day08.part_1(input), "14"); // Verify the expected result for Part 1.
    }

    /// Test for Part 2 of Day08.
    #[test]
    fn test_aoc2024_day08_part_2() {
        use crate::solutions::aoc2024::Day08;
        let day08 = Day08 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
        "#;
        assert_eq!(day08.part_2(input), "34"); // Verify the expected result for Part 2.
    }
}
