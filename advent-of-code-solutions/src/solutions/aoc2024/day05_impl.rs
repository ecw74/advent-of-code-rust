use super::day05::Day05;
use std::collections::{HashMap, HashSet, VecDeque};

impl Day05 {
    /// Parses the input into pairs (ordering rules) and lists (updates).
    fn parse_input(input: &str) -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
        let lines = input.lines();

        let mut pairs: Vec<(u8, u8)> = Vec::new(); // Stores the ordering rules
        let mut lists = Vec::new(); // Stores the page update lists
        let mut parsing_pairs = true; // Tracks whether we're reading rules or updates

        for line in lines {
            if line.trim().is_empty() {
                continue; // Skip empty lines
            }

            if parsing_pairs {
                // Parse ordering rules (lines containing '|')
                if line.contains('|') {
                    let parts: Vec<&str> = line.split('|').map(|part| part.trim()).collect();
                    if let [src, dst] = &parts[..] {
                        let src = src.parse::<u8>().unwrap();
                        let dst = dst.parse::<u8>().unwrap();
                        pairs.push((src, dst));
                    }
                } else {
                    parsing_pairs = false; // Switch to parsing update lists
                }
            }

            if !parsing_pairs {
                // Parse update lists (lines containing ',')
                let list: Vec<u8> = line
                    .split(',')
                    .map(|part| part.trim().parse::<u8>().unwrap())
                    .collect();
                lists.push(list);
            }
        }

        (pairs, lists)
    }

    /// Performs a topological sort on the given pairs (graph edges).
    /// Returns a sorted order of nodes.
    fn topological_sort(pairs: &Vec<(u8, u8)>) -> Vec<u8> {
        // Build the adjacency list and indegree map
        let mut graph: HashMap<u8, Vec<u8>> = HashMap::new();
        let mut indegree: HashMap<u8, u8> = HashMap::new();

        for &(src, dst) in pairs {
            graph.entry(src).or_insert(Vec::new()).push(dst);
            *indegree.entry(dst).or_insert(0) += 1;
            indegree.entry(src).or_insert(0); // Ensure src is included in indegree map
        }

        // Initialize a queue for nodes with zero incoming edges
        let mut queue: VecDeque<u8> = indegree
            .iter()
            .filter_map(|(&node, &deg)| if deg == 0 { Some(node) } else { None })
            .collect();

        let mut sorted: Vec<u8> = Vec::new(); // Stores the final sorted order

        while let Some(node) = queue.pop_front() {
            sorted.push(node);

            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    if let Some(deg) = indegree.get_mut(&neighbor) {
                        *deg -= 1; // Decrement indegree
                        if *deg == 0 {
                            queue.push_back(neighbor); // Add to queue when indegree is zero
                        }
                    }
                }
            }
        }

        sorted
    }

    /// Generalized function to calculate the sum of middle elements.
    /// If `only_valid` is true, it processes valid updates only;
    /// otherwise, it processes invalid updates after reordering them.
    fn calculate_middle_sum(&self, input: &str, only_valid: bool) -> u32 {
        let (pairs, lists) = Self::parse_input(input);
        let mut sum_of_middles: u32 = 0;

        for vector in lists {
            let relevant_nodes: HashSet<u8> = vector.iter().cloned().collect();
            let filtered_pairs: Vec<(u8, u8)> = pairs
                .iter()
                .filter(|&&(src, dst)| {
                    relevant_nodes.contains(&src) && relevant_nodes.contains(&dst)
                })
                .cloned()
                .collect();

            let order = Self::topological_sort(&filtered_pairs);

            let order_map: HashMap<u8, usize> =
                order.iter().enumerate().map(|(i, &n)| (n, i)).collect();

            let mut sorted_vec = vector.clone();
            sorted_vec.sort_by_key(|&x| order_map.get(&x).cloned().unwrap_or(usize::MAX));

            let is_valid = sorted_vec == vector;

            if is_valid == only_valid {
                if !sorted_vec.is_empty() {
                    let middle_index = sorted_vec.len() / 2;
                    sum_of_middles += sorted_vec[middle_index] as u32;
                }
            }
        }

        sum_of_middles
    }

    /// Solves Part 1: Computes the sum of middle elements for correctly ordered updates.
    pub fn part_1(&self, input: &str) -> String {
        self.calculate_middle_sum(input, true).to_string()
    }

    /// Solves Part 2: Computes the sum of middle elements for reordered updates.
    pub fn part_2(&self, input: &str) -> String {
        self.calculate_middle_sum(input, false).to_string()
    }
}

mod test {
    #[test]
    /// Test for part 1 of Day05 for AoC 2024.
    /// Ensures correct calculation of the sum of first and last digits in each line.
    fn test_aoc2024_day05_part_1() {
        use crate::solutions::aoc2024::Day05;
        let day05 = Day05 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
        "#;
        assert_eq!(day05.part_1(input), "143"); // Asserts if the function output matches the expected result.
    }

    #[test]
    /// Test for part 2 of Day05 for AoC 2024.
    /// Verifies correct handling and sum of lines with spelled-out numbers.
    fn test_aoc2024_day05_part_2() {
        use crate::solutions::aoc2024::Day05;
        let day05 = Day05 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
        "#;
        assert_eq!(day05.part_2(input), "123"); // Asserts if the function output matches the expected result.
    }
}
