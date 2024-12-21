use super::day21::Day21;
use crate::utils::point::Point;
use hashbrown::HashMap;
use std::cmp;

impl Day21 {
    /// Parses the input into a vector of codes. Each code corresponds to a sequence of
    /// numbers and letters that the numeric keypad robot must type.
    fn parse_codes(input: &str) -> Vec<String> {
        input
            .lines()
            .filter(|line| !line.trim().is_empty()) // Skip empty lines.
            .map(|line| line.trim().to_string()) // Trim whitespace and convert to String.
            .collect()
    }

    /// Maps a key on the numeric keypad to its position as a Point (x, y).
    /// The numeric keypad is laid out as described in the puzzle, with specific coordinates.
    fn num_pad(key: char) -> Point<i32> {
        match key {
            '7' => Point::new(0, 0),
            '8' => Point::new(0, 1),
            '9' => Point::new(0, 2),
            '4' => Point::new(1, 0),
            '5' => Point::new(1, 1),
            '6' => Point::new(1, 2),
            '1' => Point::new(2, 0),
            '2' => Point::new(2, 1),
            '3' => Point::new(2, 2),
            '0' => Point::new(3, 1),
            'A' => Point::new(3, 2),
            _ => panic!(), // Panic if an invalid key is encountered.
        }
    }

    /// Maps a key on the directional keypad to its position as a Point (x, y).
    /// This represents the layout of the keypad controlling the robot arm.
    fn control_pad(key: char) -> Point<i32> {
        match key {
            '^' => Point::new(0, 1),
            'A' => Point::new(0, 2),
            '<' => Point::new(1, 0),
            'v' => Point::new(1, 1),
            '>' => Point::new(1, 2),
            _ => panic!(), // Panic if an invalid key is encountered.
        }
    }

    /// Generates the sequence of directional commands needed to move the robot arm
    /// to the target position and press the button ('A').
    ///
    /// - `offset`: Difference between the current position and the target position.
    /// - `horizontal_first`: Whether horizontal moves should be prioritized in the sequence.
    fn build_move_sequence(offset: Point<i32>, horizontal_first: bool) -> Vec<char> {
        let (dx, dy) = (offset.x, offset.y);

        // Determine the number of vertical (up/down) and horizontal (left/right) steps required.
        let vertical_steps = dx.abs() as usize;
        let horizontal_steps = dy.abs() as usize;

        // Determine the direction of vertical and horizontal moves.
        let vertical_move = if dx > 0 { '^' } else { 'v' };
        let horizontal_move = if dy > 0 { '<' } else { '>' };

        // Create repeated move characters for the required steps.
        let vertical_moves = vec![vertical_move; vertical_steps];
        let horizontal_moves = vec![horizontal_move; horizontal_steps];

        // Combine the moves in the specified order.
        let mut moves: Vec<_> = if horizontal_first {
            horizontal_moves
                .into_iter()
                .chain(vertical_moves.into_iter())
                .collect()
        } else {
            vertical_moves
                .into_iter()
                .chain(horizontal_moves.into_iter())
                .collect()
        };

        // Add 'A' to press the button at the target position.
        moves.push('A');

        moves
    }

    /// Calculates the shortest sequence of moves required for a robot to press buttons
    /// on its directional keypad. Uses recursion to explore possible move sequences.
    ///
    /// - `offset`: Difference between the current position and the target position.
    /// - `steps`: Remaining chain levels to traverse.
    /// - `horizontal_first`: Whether horizontal moves should be prioritized.
    /// - `cache`: A memoization map to store previously calculated results.
    fn calculate_shortest_moves(
        offset: Point<i32>,
        steps: usize,
        horizontal_first: bool,
        cache: &mut HashMap<(Point<i32>, usize, bool), u64>,
    ) -> u64 {
        // Check if the result is already cached.
        let cache_key = (offset, steps, horizontal_first);
        if let Some(&cached) = cache.get(&cache_key) {
            return cached;
        }

        // Build the move sequence for the given offset.
        let moves = Self::build_move_sequence(offset, horizontal_first);

        // Base case: If no further steps remain, return the length of the move sequence.
        if steps == 0 {
            let result = moves.len() as u64;
            cache.insert(cache_key, result);
            return result;
        }

        // Recursive case: Traverse the directional keypad and calculate distances.
        let mut total_cost = 0;
        let mut current_pos = Self::control_pad('A'); // Start at the 'A' button.

        for c in moves {
            let next_pos = Self::control_pad(c); // Target position for current move.
            let prev_pos = current_pos; // Previous position.
            current_pos = next_pos; // Update the current position.
            let distance = prev_pos.distance(next_pos); // Compute the distance to the target.

            // Decide the next recursive step based on the positions.
            let step_result = if (distance.x == 0 || distance.y == 0)
                || (next_pos == Point::new(1, 0) && prev_pos.x == 0)
            {
                Self::calculate_shortest_moves(distance, steps - 1, false, cache)
            } else if prev_pos == Point::new(1, 0) && next_pos.x == 0 {
                Self::calculate_shortest_moves(distance, steps - 1, true, cache)
            } else {
                cmp::min(
                    Self::calculate_shortest_moves(distance, steps - 1, false, cache),
                    Self::calculate_shortest_moves(distance, steps - 1, true, cache),
                )
            };

            total_cost += step_result; // Accumulate the result.
        }

        // Store the result in the cache.
        cache.insert(cache_key, total_cost);
        total_cost
    }

    /// Calculates the total sequence length needed to type a code on the numeric keypad.
    /// Traverses the entire chain of robots recursively, updating the position at each step.
    fn calculate_chain_traversal(sequence: &str, steps: usize) -> u64 {
        let mut cache = HashMap::new();

        let mut current_position = Self::num_pad('A'); // Starting at 'A' on the numeric keypad.
        let mut total_cost = 0;

        // Iterate over each character in the sequence.
        for c in sequence.chars() {
            let next_pos = Self::num_pad(c); // Target position for the current key.
            let prev_pos = current_position; // Previous position.
            let distance = current_position.distance(next_pos); // Calculate the distance.
            current_position = next_pos; // Update the current position.

            // Decide the movement sequence and recursive traversal.
            let steps = if prev_pos.x == 3 && next_pos.y == 0 {
                Self::calculate_shortest_moves(distance, steps, false, &mut cache)
            } else if prev_pos.y == 0 && next_pos.x == 3 {
                Self::calculate_shortest_moves(distance, steps, true, &mut cache)
            } else {
                cmp::min(
                    Self::calculate_shortest_moves(distance, steps, true, &mut cache),
                    Self::calculate_shortest_moves(distance, steps, false, &mut cache),
                )
            };

            total_cost += steps; // Add the steps to the total.
        }

        total_cost * sequence[0..3].parse::<u64>().unwrap()
    }

    /// Solution for Part 1: Calculates the sum of complexities for the five codes
    /// with a chain length of 2 robots.
    pub fn part_1(&self, input: &str) -> String {
        let codes = Self::parse_codes(input);
        codes
            .iter()
            .map(|code| Self::calculate_chain_traversal(code, 2))
            .sum::<u64>()
            .to_string()
    }

    /// Solution for Part 2: Calculates the sum of complexities for the five codes
    /// with a chain length of 25 robots.
    pub fn part_2(&self, input: &str) -> String {
        let codes = Self::parse_codes(input);
        codes
            .iter()
            .map(|code| Self::calculate_chain_traversal(code, 25))
            .sum::<u64>()
            .to_string()
    }
}

mod test {
    #[test]
    /// Tests Part 1 of Day 21. Verifies the correct calculation of complexities for
    /// the given codes with a chain of 2 robots.
    fn test_aoc2024_day21_part_1() {
        use crate::solutions::aoc2024::Day21;

        let day21 = Day21 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };

        // Input representing the five codes to process.
        let input = r#"
029A
980A
179A
456A
379A
        "#;

        // Ensure the output matches the expected sum of complexities.
        assert_eq!(day21.part_1(input), "126384");
    }

    #[test]
    /// Tests Part 2 of Day 21. Verifies correct handling of a longer chain (25 robots)
    /// and accurate complexity calculations for the same codes.
    fn test_aoc2024_day21_part_2() {
        use crate::solutions::aoc2024::Day21;

        let day21 = Day21 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };

        // Input representing the five codes to process.
        let input = r#"
029A
980A
179A
456A
379A
        "#;

        // Ensure the output matches the expected sum of complexities.
        assert_eq!(day21.part_2(input), "154115708116294");
    }
}
