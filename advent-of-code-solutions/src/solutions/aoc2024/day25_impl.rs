use super::day25::Day25;

impl Day25 {
    /// Parses the input into two separate collections: locks and keys.
    /// Each block in the input is 7 lines, where the first and last lines
    /// determine if it is a lock or a key, and the middle lines represent
    /// the block's structure as a 5x5 grid.
    fn parse_blocks(input: &str) -> (Vec<u64>, Vec<u64>) {
        let mut locks: Vec<u64> = Vec::new(); // Stores parsed locks as 64-bit values.
        let mut keys: Vec<u64> = Vec::new();  // Stores parsed keys as 64-bit values.

        // Filters out empty lines from the input.
        let lines: Vec<&str> = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();

        // Processes blocks of 7 lines at a time.
        for chunk in lines.chunks(7) {
            if chunk.len() != 7 {
                continue; // Skip invalid blocks that are not 7 lines.
            }

            // Determine if the block is a lock or key based on its first and last lines.
            let is_lock = chunk[0] == "#####" && chunk[6] == ".....";
            let is_key = chunk[0] == "....." && chunk[6] == "#####";

            if !is_lock && !is_key {
                continue; // Skip invalid blocks that do not match either pattern.
            }

            // Extracts the grid content from the middle 5 lines.
            let content = &chunk[1..6];

            // Converts the block's grid to a single 64-bit integer.
            let mut block_value: u64 = 0;

            // Iterate over each column in the grid.
            for (col, _) in content[0].chars().enumerate() {
                let mut column_bits: u8 = 0;

                // Build the bit representation for the current column.
                for (row, line) in content.iter().enumerate() {
                    if let Some(c) = line.chars().nth(col) {
                        if c == '#' {
                            column_bits |= 1 << row; // Set the bit if the character is '#'.
                        }
                    }
                }

                // Shift the column's bit representation into the correct position in the u64 value.
                block_value |= (column_bits as u64) << (col * 8);
            }

            // Add the parsed block to the appropriate vector (locks or keys).
            if is_lock {
                locks.push(block_value);
            } else if is_key {
                keys.push(block_value);
            }
        }

        (locks, keys)
    }

    /// Solves part 1 of the puzzle by determining the number of valid lock-key pairs.
    /// A lock-key pair is valid if their bitwise AND result is 0, meaning no overlapping pins.
    pub fn part_1(&self, input: &str) -> String {
        let (locks, keys) = Self::parse_blocks(input); // Parse locks and keys from the input.
        let (smaller, larger, _is_keys_smaller) = if keys.len() <= locks.len() {
            (keys, locks, true) // Use keys as the smaller list if they have fewer elements.
        } else {
            (locks, keys, false) // Otherwise, use locks as the smaller list.
        };

        // Reduce the smaller list to its lower 40 bits and sort it.
        let mut sorted_smaller: Vec<u64> = smaller.iter().map(|&x| x & 0xFFFFFFFFFF).collect();
        sorted_smaller.sort_unstable();

        // Count valid combinations of lock-key pairs.
        larger
            .iter()
            .map(|&larger_elem| {
                let larger_code = larger_elem & 0xFFFFFFFFFF; // Use only the lower 40 bits of the lock.
                // Check against all elements in the smaller list.
                sorted_smaller
                    .iter()
                    .filter(|&&smaller_code| (smaller_code & larger_code) == 0) // No overlapping pins.
                    .count()
            })
            .sum::<usize>() // Sum up the counts for all locks.
            .to_string()
    }

    /// Placeholder for part 2 of the puzzle, returning an empty string.
    pub fn part_2(&self, _input: &str) -> String {
        "".to_string()
    }
}

mod test {
    #[test]
    /// Test for part 1 of Day25 for AoC 2024.
    /// Ensures correct calculation of valid lock-key pair counts.
    fn test_aoc2024_day25_part_1() {
        use crate::solutions::aoc2024::Day25;
        let day25 = Day25 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
        "#;
        assert_eq!(day25.part_1(input), "3"); // Asserts if the function output matches the expected result.
    }

    #[test]
    /// Test for part 2 of Day25 for AoC 2024.
    /// Verifies correct handling of input and outputs for part 2 (currently empty).
    fn test_aoc2024_day25_part_2() {
        use crate::solutions::aoc2024::Day25;
        let day25 = Day25 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#""#;
        assert_eq!(day25.part_2(input), ""); // Asserts if the function output matches the expected result.
    }
}
