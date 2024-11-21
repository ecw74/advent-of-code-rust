use super::day03::Day03;

impl Day03 {
    // Finds the priority of the common item type that appears in all three given rucksacks.
    // This function helps determine which item type needs to be rearranged.
    fn find_common_char_priority(s1: &str, s2: &str, s3: &str) -> Option<i64> {
        // Bitmasks to track lowercase and uppercase item types in each rucksack.
        let mut lower_mask1 = 0u64; // Bitmask for lowercase items in the first rucksack
        let mut upper_mask1 = 0u64; // Bitmask for uppercase items in the first rucksack

        let mut lower_mask2 = 0u64; // Bitmask for lowercase items in the second rucksack
        let mut upper_mask2 = 0u64; // Bitmask for uppercase items in the second rucksack

        let mut lower_mask3 = 0u64; // Bitmask for lowercase items in the third rucksack
        let mut upper_mask3 = 0u64; // Bitmask for uppercase items in the third rucksack

        // Create bitmasks for the first rucksack
        for ch in s1.chars() {
            if ch.is_ascii_lowercase() {
                lower_mask1 |= 1 << (ch as u32 - 'a' as u32);
            } else if ch.is_ascii_uppercase() {
                upper_mask1 |= 1 << (ch as u32 - 'A' as u32);
            }
        }

        // Create bitmasks for the second rucksack
        for ch in s2.chars() {
            if ch.is_ascii_lowercase() {
                lower_mask2 |= 1 << (ch as u32 - 'a' as u32);
            } else if ch.is_ascii_uppercase() {
                upper_mask2 |= 1 << (ch as u32 - 'A' as u32);
            }
        }

        // Create bitmasks for the third rucksack
        for ch in s3.chars() {
            if ch.is_ascii_lowercase() {
                lower_mask3 |= 1 << (ch as u32 - 'a' as u32);
            } else if ch.is_ascii_uppercase() {
                upper_mask3 |= 1 << (ch as u32 - 'A' as u32);
            }
        }

        // Find the common item type among the three rucksacks (lowercase first)
        let common_lower_mask = lower_mask1 & lower_mask2 & lower_mask3;
        if common_lower_mask != 0 {
            let pos = common_lower_mask.trailing_zeros();
            return Some((pos as i64) + 1); // Priorities for a-z: 1-26
        }

        // Find the common item type among the three rucksacks (uppercase)
        let common_upper_mask = upper_mask1 & upper_mask2 & upper_mask3;
        if common_upper_mask != 0 {
            let pos = common_upper_mask.trailing_zeros();
            return Some((pos as i64) + 27); // Priorities for A-Z: 27-52
        }

        None
    }

    // Solves Part 1 of the Rucksack Reorganization puzzle.
    // Finds the sum of priorities of misplaced items within each rucksack.
    pub fn part_1(&self, _input: &str) -> String {
        let sum = _input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let mid = line.len() / 2; // Calculate the midpoint of the rucksack
                let (left, right) = line.split_at(mid); // Split the rucksack into two compartments
                Self::find_common_char_priority(left, right, right).unwrap_or(0)
            })
            .sum::<i64>(); // Sum the priorities of the misplaced items
        sum.to_string() // Convert the final result to a String
    }

    // Solves Part 2 of the Rucksack Reorganization puzzle.
    // Finds the sum of priorities of common badge items across groups of three Elves.
    pub fn part_2(&self, _input: &str) -> String {
        let lines: Vec<&str> = _input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();

        let sum = lines
            .chunks(3) // Group the rucksacks in sets of three (each Elf group)
            .filter_map(|chunk| {
                if chunk.len() == 3 {
                    Self::find_common_char_priority(chunk[0], chunk[1], chunk[2])
                } else {
                    None // Ignore groups that do not have exactly three rucksacks
                }
            })
            .sum::<i64>(); // Sum the priorities of the badge items
        sum.to_string() // Convert the final result to a String
    }
}

mod test {
    #[test]
    /// Test for Part 1 of Day 3: Rucksack Reorganization.
    /// Ensures that the sum of misplaced item priorities is calculated correctly.
    fn test_aoc2022_day03_part_1() {
        use crate::solutions::aoc2022::Day03;
        let day03 = Day03 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
        "#;
        assert_eq!(day03.part_1(input), "157"); // Check if the result matches the expected value
    }

    #[test]
    /// Test for Part 2 of Day 3: Rucksack Reorganization.
    /// Verifies the correct calculation of badge priorities for groups of three Elves.
    fn test_aoc2022_day03_part_2() {
        use crate::solutions::aoc2022::Day03;
        let day03 = Day03 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
        "#;
        assert_eq!(day03.part_2(input), "70"); // Check if the result matches the expected value
    }
}
