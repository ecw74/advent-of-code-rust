use super::day05::Day05;

impl Day05 {
    /// Parses the input string to extract the initial state of stacks and the move instructions.
    /// Returns a tuple containing:
    /// - A vector of vectors of chars representing stacks (each stack is a vector where the bottom of the stack is the first element).
    /// - A vector of move instructions, where each instruction is an array `[count, from, to]`:
    ///   - `count`: Number of crates to move.
    ///   - `from`: Source stack index (1-based).
    ///   - `to`: Destination stack index (1-based).
    fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<[usize; 3]>) {
        let mut lines = input.lines();

        // Determine the number of stacks by finding the line with stack numbering (contains digits).
        let mut stack_count = 0;
        while let Some(line) = lines.next() {
            if line.chars().any(|c| c.is_digit(10)) {
                stack_count = line.split_whitespace().count();
                break;
            }
        }

        // Initialize the stacks as empty vectors with pre-allocated capacity.
        let mut stacks: Vec<Vec<char>> = vec![Vec::with_capacity(100); stack_count];

        // Reset lines iterator to process the stack representation again.
        lines = input.lines();
        while let Some(line) = lines.next() {
            // Stop parsing the stack representation if a line with digits (stack numbering) is reached.
            if line.chars().any(|c| c.is_digit(10)) {
                break;
            }

            // Parse the characters in the line and assign them to the appropriate stacks.
            let chars: Vec<char> = line.chars().collect();
            for (i, &c) in chars.iter().enumerate() {
                if c != ' ' && (i % 4 == 1) {
                    // Use the character position to determine the corresponding stack index.
                    let column_index = i / 4;
                    stacks[column_index].insert(0, c); // Insert crates at the bottom of the stack.
                }
            }
        }

        let result_stacks = stacks;

        // Parse the move instructions from the remaining lines.
        let moves: Vec<[usize; 3]> = lines
            .filter(|line| line.starts_with("move"))
            .map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                [
                    parts[1].parse::<usize>().unwrap(), // Number of crates to move.
                    parts[3].parse::<usize>().unwrap(), // Source stack (1-based).
                    parts[5].parse::<usize>().unwrap(), // Destination stack (1-based).
                ]
            })
            .collect();

        (result_stacks, moves)
    }

    /// Executes the move instructions to rearrange the stacks.
    /// Parameters:
    /// - `stacks`: Mutable reference to the vector of stacks.
    /// - `moves`: Slice of move instructions.
    /// - `reverse`: If `true`, crates are moved in reverse order; otherwise, they retain their order.
    fn execute_moves(stacks: &mut Vec<Vec<char>>, moves: &[[usize; 3]], reverse: bool) {
        for mov in moves {
            let count = mov[0]; // Number of crates to move.
            let from = mov[1] - 1; // Convert 1-based index to 0-based for the source stack.
            let to = mov[2] - 1; // Convert 1-based index to 0-based for the destination stack.

            // Split stacks to safely access mutable references for `from` and `to`.
            let (source, target) = if from < to {
                let (left, right) = stacks.split_at_mut(to);
                (&mut left[from], &mut right[0])
            } else {
                let (left, right) = stacks.split_at_mut(from);
                (&mut right[0], &mut left[to])
            };

            // Move crates directly without creating intermediate Strings or Vecs.
            let split_index = source.len() - count;
            if reverse {
                target.extend(source.drain(split_index..).rev());
            } else {
                target.extend(source.drain(split_index..));
            }
        }
    }

    /// Solves part 1 of the puzzle where crates are moved in reverse order.
    pub fn part_1(&self, _input: &str) -> String {
        let (mut stacks, moves) = Self::parse_input(_input);
        Self::execute_moves(&mut stacks, &moves, true); // Reverse order for CrateMover 9000.

        // Collect the top crate from each stack to form the final result.
        stacks.iter().filter_map(|s| s.last().copied()).collect()
    }

    /// Solves part 2 of the puzzle where crates retain their order when moved.
    pub fn part_2(&self, _input: &str) -> String {
        let (mut stacks, moves) = Self::parse_input(_input);
        Self::execute_moves(&mut stacks, &moves, false); // Retain order for CrateMover 9001.

        // Collect the top crate from each stack to form the final result.
        stacks.iter().filter_map(|s| s.last().copied()).collect()
    }
}


mod test {
    #[test]
    /// Test for part 1 of Day 5. Verifies correct crate rearrangement using CrateMover 9000.
    fn test_aoc2022_day05_part_1() {
        use crate::solutions::aoc2022::Day05;
        let day05 = Day05 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;
        assert_eq!(day05.part_1(input), "CMZ");
    }

    #[test]
    /// Test for part 2 of Day 5. Verifies correct crate rearrangement using CrateMover 9001.
    fn test_aoc2022_day05_part_2() {
        use crate::solutions::aoc2022::Day05;
        let day05 = Day05 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;
        assert_eq!(day05.part_2(input), "MCD");
    }
}
