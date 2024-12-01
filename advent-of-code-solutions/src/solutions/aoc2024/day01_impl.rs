use std::collections::HashMap;
use super::day01::Day01;

// Implementation of methods for the Day01 struct
impl Day01 {
    /// Parses the input string into two vectors of integers.
    /// Each line in the input is expected to contain two numbers separated by whitespace.
    fn parse_to_vectors(input: &str) -> (Vec<i32>, Vec<i32>) {
        let pairs: Vec<(i32, i32)> = input
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() == 2 {
                    Some((parts[0].parse::<i32>().ok()?, parts[1].parse::<i32>().ok()?))
                } else {
                    None
                }
            })
            .collect();

        // Separate and sort both lists.
        let (mut left, mut right): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
        left.sort_unstable();
        right.sort_unstable();

        (left, right)
    }

    /// Solves Part 1 of the puzzle.
    /// Calculates the total distance between paired values of sorted left and right lists.
    pub fn part_1(&self, _input: &str) -> String {
        // Parse the input into two vectors.
        let (left, right) = Self::parse_to_vectors(_input);

        // Calculate the total distance by summing the absolute differences of paired elements.
        left.iter()
            .zip(right.iter()) // Pair elements from left and right vectors.
            .map(|(l, r)| (l - r).abs() as i32) // Calculate absolute difference for each pair.
            .sum::<i32>()
            .to_string()
    }

    /// Solves Part 2 of the puzzle.
    /// Calculates the similarity score based on the frequency of elements in the right list.
    pub fn part_2(&self, _input: &str) -> String {
        let (left, right) = Self::parse_to_vectors(_input);

        // Pre-calc occurrences in the right vector (this is for runtime optimization)
        let right_counts: HashMap<i32, i32> =
            right.into_iter().fold(HashMap::new(), |mut map, num| {
                *map.entry(num).or_insert(0) += 1;
                map
            });

        // Calculate the similarity score.
        left.iter()
            .map(|num| *num * right_counts.get(num).copied().unwrap_or(0))
            .sum::<i32>()
            .to_string()
    }
}

// Unit tests for the implementation.
mod test {
    #[test]
    /// Test for part 1 of Day01 for AoC 2024.
    /// Ensures correct calculation of the total distance between paired values.
    fn test_aoc2024_day01_part_1() {
        use crate::solutions::aoc2024::Day01;
        let day01 = Day01 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
3   4
4   3
2   5
1   3
3   9
3   3
        "#;
        assert_eq!(day01.part_1(input), "11"); // Assert correct output for example input.
    }

    #[test]
    /// Test for part 2 of Day01 for AoC 2024.
    /// Verifies correct calculation of the similarity score.
    fn test_aoc2024_day01_part_2() {
        use crate::solutions::aoc2024::Day01;
        let day01 = Day01 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
3   4
4   3
2   5
1   3
3   9
3   3
        "#;
        assert_eq!(day01.part_2(input), "31"); // Assert correct output for example input.
    }
}
