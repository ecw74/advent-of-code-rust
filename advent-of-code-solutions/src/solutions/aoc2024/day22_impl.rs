use super::day22::Day22;

impl Day22 {
    /// Parses a string input to extract valid numbers (u32) from each line.
    /// Lines that fail to parse (e.g., non-numeric content) are ignored.
    fn parse_numbers(input: &str) -> Vec<u32> {
        input
            .lines()
            .filter_map(|line| line.trim().parse::<u32>().ok()) // Attempt to parse each line, ignoring invalid ones
            .collect() // Collect the successfully parsed numbers into a vector
    }

    /// Computes the next secret number in the pseudorandom sequence.
    /// Uses bitwise transformations (XOR) combined with modulo 2^24 to keep values within a 24-bit range.
    /// Optimized version of following code:
    // secret ^= secret << 6;  // secret XOR (secret * 64)
    // secret &= 0xFFFFFF;     // secret mod 16777216
    // secret ^= secret >> 5;  // secret XOR (secret / 32)
    // secret &= 0xFFFFFF;     // secret mod 16777216
    // secret ^= secret << 11; // secret XOR (secret * 2048)
    // secret &= 0xFFFFFF;     // secret mod 16777216
    #[inline(always)]
    fn next_secret(secret: u32) -> u32 {
        let a = secret ^ (secret << 6);
        let b = a & 0xFFFFFF;
        let c = b ^ (b >> 5);
        (c ^ ((c << 11) & 0xFFFFFF)) & 0xFFFFFF
    }

    /// Generates a sequence of pseudorandom secret numbers starting from an initial value.
    /// Includes the initial value and `count` additional numbers in the sequence.
    #[inline(always)]
    fn generate_secret_numbers(initial: u32, count: usize) -> Vec<u32> {
        let mut secrets = Vec::with_capacity(count + 1); // Preallocate space for performance
        let mut current = initial;
        secrets.push(current); // Start the sequence with the initial value
        for _ in 0..count {
            current = Self::next_secret(current); // Compute the next secret in the sequence
            secrets.push(current); // Add it to the vector
        }
        secrets
    }

    /// Similar to `generate_secret_numbers`, but computes the modulo 10 of each secret.
    /// Produces a sequence of `i8` values to reduce memory usage.
    #[inline(always)]
    fn generate_secret_numbers_mod10(initial: u32, count: usize) -> Vec<i8> {
        let mut secrets = Vec::with_capacity(count + 1); // Preallocate space for performance
        let mut current = initial;
        secrets.push((current % 10) as i8); // Compute the modulo 10 of the initial value
        for _ in 0..count {
            current = Self::next_secret(current); // Compute the next secret in the sequence
            secrets.push((current % 10) as i8); // Compute the modulo 10 and add to the vector
        }
        secrets
    }

    /// Part 1: Calculates the sum of the 2000th secret number in each buyer's sequence.
    /// Parses the input to obtain initial secrets, computes sequences, and sums the results.
    pub fn part_1(&self, input: &str) -> String {
        let initial_secrets = Self::parse_numbers(input); // Extract initial secrets from input
        let mut total_sum: u64 = 0; // Use u64 to handle potential overflow during summation

        for secret in initial_secrets {
            let secrets = Self::generate_secret_numbers(secret, 2000); // Generate a sequence of 2001 numbers
            total_sum += *secrets.last().unwrap_or(&0) as u64; // Add the last number (2000th secret) to the total sum
        }

        total_sum.to_string() // Convert the result to a string for output
    }

    /// Part 2: Calculates the sum of sale prices based on unique delta sequences.
    /// Finds unique 4-element delta sequences in modulo-10 secret numbers, computes indices, and sums prices.
    pub fn part_2(&self, input: &str) -> String {
        let initial_secrets = Self::parse_numbers(input); // Parse initial secrets from input
        let mut sale_prices = vec![0u32; 130321]; // Array to store sales prices for each delta sequence
        let mut seen_tuples = vec![false; 130321]; // Bitset to track which delta sequences have been processed

        for secret in initial_secrets {
            let secrets = Self::generate_secret_numbers_mod10(secret, 2000); // Generate modulo-10 sequence
            seen_tuples.iter_mut().for_each(|x| *x = false); // Reset the bitset for this buyer's sequence

            for i in 0..(secrets.len() - 4) {
                // Compute 4-element deltas
                let delta1 = secrets[i] - secrets[i + 1];
                let delta2 = secrets[i + 1] - secrets[i + 2];
                let delta3 = secrets[i + 2] - secrets[i + 3];
                let delta4 = secrets[i + 3] - secrets[i + 4];

                // Compute the unique index for the delta sequence
                let index = ((delta1 + 9) as usize * 19 * 19 * 19)
                    + ((delta2 + 9) as usize * 19 * 19)
                    + ((delta3 + 9) as usize * 19)
                    + (delta4 + 9) as usize;

                // If this delta sequence has not been seen before, add to sale prices
                if !seen_tuples[index] {
                    seen_tuples[index] = true; // Mark as seen
                    sale_prices[index] += secrets[i + 4] as u32; // Add the next secret number to the sale price
                }
            }
        }

        let max_total_sales = sale_prices.into_iter().max().unwrap_or(0); // Find the maximum sale price
        max_total_sales.to_string() // Convert the result to a string for output
    }
}

mod test {
    #[test]
    /// Test for Part 1: Ensures the correct sum of the 2000th secret numbers is calculated.
    fn test_aoc2024_day22_part_1() {
        use crate::solutions::aoc2024::Day22;
        let day22 = Day22 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"cha
1
10
100
2024
        "#;
        assert_eq!(day22.part_1(input), "37327623"); // Check if Part 1 produces the expected result
    }

    #[test]
    /// Test for Part 2: Verifies the correct sum of sales for the optimal delta sequence.
    fn test_aoc2024_day22_part_2() {
        use crate::solutions::aoc2024::Day22;
        let day22 = Day22 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
1
2
3
2024
        "#;
        assert_eq!(day22.part_2(input), "23"); // Check if Part 2 produces the expected result
    }
}
