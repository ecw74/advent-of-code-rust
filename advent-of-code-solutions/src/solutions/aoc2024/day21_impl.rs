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
    fn numpad(key: char) -> Point<i32> {
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
    fn arrowpad(key: char) -> Point<i32> {
        match key {
            '^' => Point::new(0, 1),
            'A' => Point::new(0, 2),
            '<' => Point::new(1, 0),
            'v' => Point::new(1, 1),
            '>' => Point::new(1, 2),
            _ => panic!(), // Panic if an invalid key is encountered.
        }
    }

    /// Calculates the shortest sequence of moves required for a robot to press buttons
    /// on its directional keypad. Uses recursion to explore possible move sequences.
    ///
    /// - `i`: Current position difference as a Point.
    /// - `steps`: Remaining chain levels to traverse.
    /// - `h_first`: Whether to prioritize horizontal moves.
    /// - `cache`: A memoization map to store previously calculated results.
    fn do_arrows(
        i: Point<i32>,
        steps: usize,
        h_first: bool,
        cache: &mut HashMap<(Point<i32>, usize, bool), u64>,
    ) -> u64 {
        let key = (i, steps, h_first); // Unique key for caching results.
        let result;

        // Compute absolute distances for x and y axes.
        let abs = i.abs();
        let ii = abs.x as usize;
        let jj = abs.y as usize;

        // Generate the sequence of moves based on the distances and direction prioritization.
        let mut chunk = vec![if i.x > 0 { '^' } else { 'v' }; ii]; // Vertical moves.
        chunk.extend(vec![if i.y > 0 { '<' } else { '>' }; jj]); // Horizontal moves.

        if h_first {
            chunk.reverse(); // Reverse order if horizontal moves are prioritized.
        }

        chunk.push('A'); // Add the press action.

        // Return the cached result if available.
        if let Some(&result) = cache.get(&key) {
            return result;
        }

        // Base case: No more steps, return the length of the move sequence.
        if steps == 0 {
            result = chunk.len() as u64;
        } else {
            // Recursive case: Traverse the directional keypad and calculate distances.
            let mut loc = Self::arrowpad('A'); // Start at the 'A' button.
            result = chunk
                .into_iter()
                .map(|c| {
                    let n = Self::arrowpad(c); // Target position for current move.
                    let p = loc; // Previous position.
                    loc = n; // Update the current position.
                    let d = p.distance(n); // Compute the distance to the target.

                    // Decide the next recursive step based on the positions.
                    if d.x == 0 || d.y == 0 {
                        Self::do_arrows(d, steps - 1, false, cache)
                    } else if n == Point::new(1, 0) && p.x == 0 {
                        Self::do_arrows(d, steps - 1, false, cache)
                    } else if p == Point::new(1, 0) && n.x == 0 {
                        Self::do_arrows(d, steps - 1, true, cache)
                    } else {
                        cmp::min(
                            Self::do_arrows(d, steps - 1, false, cache),
                            Self::do_arrows(d, steps - 1, true, cache),
                        )
                    }
                })
                .sum::<u64>();
        }

        // Store the result in the cache.
        cache.insert(key, result);
        result
    }

    /// Calculates the total sequence length needed to type a code on the numeric keypad.
    /// Traverses the entire chain of robots recursively, updating the position at each step.
    fn enter_sequence(sequence: &str, steps: usize) -> u64 {
        let mut loc = Self::numpad('A'); // Starting at 'A' on the numeric keypad.
        let mut cache = HashMap::new();

        // Multiply the numeric part of the code by the sequence length.
        sequence[0..3].parse::<u64>().unwrap()
            * sequence
                .chars()
                .map(|c| {
                    let n = Self::numpad(c); // Target position for the current key.
                    let p = loc; // Previous position.
                    let d = loc.distance(n); // Calculate the distance.
                    loc = n; // Update the current position.

                    // Decide the movement sequence and recursive traversal.
                    if p.x == 3 && n.y == 0 {
                        Self::do_arrows(d, steps, false, &mut cache)
                    } else if p.y == 0 && n.x == 3 {
                        Self::do_arrows(d, steps, true, &mut cache)
                    } else {
                        cmp::min(
                            Self::do_arrows(d, steps, true, &mut cache),
                            Self::do_arrows(d, steps, false, &mut cache),
                        )
                    }
                })
                .sum::<u64>() // Sum up the sequence lengths for the code.
    }

    /// Solution for Part 1: Calculates the sum of complexities for the five codes
    /// with a chain length of 2 robots.
    pub fn part_1(&self, input: &str) -> String {
        let codes = Self::parse_codes(input);
        codes
            .iter()
            .map(|code| Self::enter_sequence(code, 2))
            .sum::<u64>()
            .to_string()
    }

    /// Solution for Part 2: Calculates the sum of complexities for the five codes
    /// with a chain length of 25 robots.
    pub fn part_2(&self, input: &str) -> String {
        let codes = Self::parse_codes(input);
        codes
            .iter()
            .map(|code| Self::enter_sequence(code, 25))
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
