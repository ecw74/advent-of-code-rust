use super::day09::Day09;

impl Day09 {
    /// Parses the input for Part 1, returning a vector of blocks as (empty: bool, id: usize).
    /// - `empty`: Indicates if the block is free space (`true`) or a file (`false`).
    /// - `id`: The file ID, if the block is part of a file.
    fn parse_input_for_part_one(input: &str) -> Vec<(bool, usize)> {
        let mut id = 0; // File ID counter
        let mut free = false; // Tracks whether the current segment represents free space
        let mut filesystem = Vec::new(); // Holds the parsed representation of the filesystem

        // Process each character in the input string
        for c in input.chars().filter(|c| c.is_ascii_digit()) {
            let n = c.to_digit(10).unwrap() as usize; // Convert character to a number
            match free {
                true => filesystem.extend((0..n).map(|_| (true, 0))), // Add `n` free space blocks
                false => {
                    filesystem.extend((0..n).map(|_| (false, id))); // Add `n` file blocks with the current ID
                    id += 1; // Increment file ID for the next file
                }
            }
            free = !free; // Toggle between free space and file segments
        }
        filesystem
    }

    /// Parses the input for Part 2, returning a vector of files and free spaces.
    /// - Files are represented as `(id, position, length)`.
    /// - Free spaces are represented as `(position, length)`.
    fn parse_input_for_part_two(input: &str) -> (Vec<(usize, usize, usize)>, Vec<(usize, usize)>) {
        let mut id = 0; // File ID counter
        let mut pos = 0; // Current position on the disk
        let mut free = false; // Tracks whether the current segment represents free space
        let mut files = Vec::with_capacity(input.len() / 2); // Holds file metadata
        let mut free_space = Vec::with_capacity(input.len() / 2); // Holds free space metadata

        // Process each character in the input string
        for c in input.chars().filter(|c| c.is_ascii_digit()) {
            let len = c.to_digit(10).unwrap() as usize; // Convert character to a number
            match free {
                true => free_space.push((pos, len)), // Record free space at the current position
                false => {
                    files.push((id, pos, len)); // Record file metadata
                    id += 1; // Increment file ID for the next file
                }
            }
            free = !free; // Toggle between free space and file segments
            pos += len; // Move the position forward by the segment length
        }
        (files, free_space)
    }

    /// Solves Part 1 of the problem.
    /// - Moves blocks one at a time to compact the disk and calculates the resulting checksum.
    pub fn part_1(&self, input: &str) -> String {
        let mut filesystem = Self::parse_input_for_part_one(input); // Parse input into blocks
        let mut front = 0; // Pointer to the leftmost block
        let mut back = filesystem.len() - 1; // Pointer to the rightmost block

        // Compact the filesystem by moving file blocks to the leftmost free space
        while front < back {
            if !filesystem[front].0 { // If `front` points to a file block, move the pointer
                front += 1;
                continue;
            }
            if filesystem[back].0 { // If `back` points to a free space, move the pointer
                back -= 1;
                continue;
            }
            filesystem.swap(front, back); // Swap the blocks
            front += 1;
            back -= 1;
        }

        // Calculate checksum
        let checksum: u64 = filesystem
            .iter()
            .enumerate() // Iterate over the blocks with their indices
            .map(|(index, &(empty, id))| {
                match empty {
                    true => 0, // Skip free space blocks
                    false => (index * id) as u64, // Multiply index by file ID
                }
            })
            .sum(); // Sum the products

        checksum.to_string()
    }

    /// Solves Part 2 of the problem.
    /// - Moves whole files to the leftmost free space that can accommodate them and calculates the checksum.
    pub fn part_2(&self, input: &str) -> String {
        let (mut files, mut free_space) = Self::parse_input_for_part_two(input); // Parse input into files and free spaces

        // Iterate over files in reverse order (starting with the largest ID)
        for file in files.iter_mut().rev() {
            for free in free_space.iter_mut() {
                if free.1 < file.2 { // If the free space is too small, skip it
                    continue;
                } else if file.1 < free.0 { // If the free space is after the file's position, stop searching
                    break;
                }
                free.1 -= file.2; // Reduce the free space by the file size
                file.1 = free.0; // Update the file's position
                free.0 += file.2; // Move the free space's starting position
                break; // Move to the next file
            }
        }

        // Calculate checksum
        let answer: u64 = files
            .iter()
            .map(|&(id, start, len)| {
                (start..start + len) // Iterate over the file's block positions
                    .map(|i| (id * i) as u64) // Multiply each position by the file ID
                    .sum::<u64>() // Sum the products for this file
            })
            .sum(); // Sum the checksums for all files

        answer.to_string()
    }
}

mod test {
    #[test]
    /// Test for part 1 of Day09 for AoC 2024.
    /// Ensures correct calculation of the checksum after compacting the filesystem by moving blocks.
    fn test_aoc2024_day09_part_1() {
        use crate::solutions::aoc2024::Day09;
        let day09 = Day09 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
2333133121414131402
"#;
        assert_eq!(day09.part_1(input), "1928"); // Expected result for part 1
    }

    #[test]
    /// Test for part 2 of Day09 for AoC 2024.
    /// Verifies correct checksum calculation after compacting by moving whole files.
    fn test_aoc2024_day09_part_2() {
        use crate::solutions::aoc2024::Day09;
        let day09 = Day09 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
2333133121414131402
"#;
        assert_eq!(day09.part_2(input), "2858"); // Expected result for part 2
    }
}
