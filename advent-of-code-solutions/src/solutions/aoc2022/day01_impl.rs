use super::day01::Day01;

impl Day01 {
    /// Calculates the total calories for each elf based on their inventory.
    /// Each paragraph in the input represents one elf's inventory.
    fn calculate_calories(&self, input: &str) -> Vec<i32> {
        input
            .split("\n\n") // Splits the input into groups, each group for one elf.
            .map(
                |elf_inventory| {
                    elf_inventory
                        .lines() // Processes each line in a group.
                        .map(|calories| calories.parse::<i32>().unwrap_or(0)) // Parses each line as an integer, defaults to 0 on failure.
                        .sum::<i32>()
                }, // Sums up the calories for the current elf.
            )
            .collect() // Collects the sums into a vector, one sum per elf.
    }

    /// Part 1 of the challenge: Finds the elf with the maximum calorie intake.
    pub fn part_1(&self, input: &str) -> String {
        let calories_vec = self.calculate_calories(input); // Gets the calorie sums.
        let sum = calories_vec
            .into_iter() // Converts vector into iterator.
            .max() // Finds the maximum sum.
            .unwrap_or(0); // Returns the max, or 0 if there are no sums.
        sum.to_string() // Convert the final result to a String
    }

    /// Part 2 of the challenge: Sums the top three calorie intakes.
    pub fn part_2(&self, input: &str) -> String {
        let mut calories_vec = self.calculate_calories(input); // Gets the calorie sums.
        calories_vec.sort_by(|a, b| b.cmp(a)); // Sorts the sums in descending order.
        let sum = calories_vec
            .iter() // Iterates over the sorted sums.
            .take(3) // Takes the top three sums.
            .sum::<i32>();
        sum.to_string() // Convert the final result to a String
    }
}

mod test {
    #[test]
    /// Test for part 1 of Day01 of AoC 2022.
    /// Ensures correct calculation of the maximum calorie intake.
    fn test_aoc2022_day01_part_1() {
        use crate::solutions::aoc2022::Day01;
        let day01 = Day01 {
            day: 1,
            year: 22,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
        "#;
        assert_eq!(day01.part_1(input), "24000"); // Asserts if the function output matches the expected result.
    }

    #[test]
    /// Test for part 2 of Day01 of AoC 2022.
    /// Verifies correct sum of the top three calorie intakes.
    fn test_aoc2022_day01_part_2() {
        use crate::solutions::aoc2022::Day01;
        let day01 = Day01 {
            day: 1,
            year: 22,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
        "#;
        assert_eq!(day01.part_2(input), "45000"); // Asserts if the function output matches the expected result.
    }
}
