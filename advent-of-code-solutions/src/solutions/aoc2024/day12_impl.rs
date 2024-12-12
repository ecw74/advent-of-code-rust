use super::day12::Day12;
use std::collections::HashSet;

impl Day12 {
    // Directions for moving up, down, left, or right on the grid
    const DIR: [(i32, i32); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

    /// Parses the input string into a 2D vector of characters representing the garden plot map.
    pub fn parse_input(input: &str) -> Vec<Vec<char>> {
        input
            .lines()
            .filter(|line| !line.trim().is_empty()) // Ignore empty lines
            .map(|line| line.chars().collect()) // Convert each line into a vector of characters
            .collect() // Collect all lines into a 2D vector
    }

    /// Calculates the total price for both area-based and side-based costs.
    ///
    /// # Arguments
    /// - `grid`: A reference to a 2D vector of characters representing the garden map.
    ///
    /// # Returns
    /// - A tuple `(total_price_area, total_price_sides)`:
    ///   - `total_price_area`: Total cost based on area and perimeter.
    ///   - `total_price_sides`: Total cost based on area and sides.
    fn calculate_price(grid: &Vec<Vec<char>>) -> (u32, u32) {
        let rows = grid.len(); // Number of rows in the grid
        let cols = grid[0].len(); // Number of columns in the grid
        let mut visited = vec![vec![false; cols]; rows]; // Track visited cells
        let mut total_price_area = 0u32; // Sum of prices based on area and perimeter
        let mut total_price_sides = 0u32; // Sum of prices based on area and sides

        // Iterate over all cells in the grid
        for i in 0..rows {
            for j in 0..cols {
                if !visited[i][j] {
                    // Start a flood-fill for a new region
                    let mut stack = vec![(i, j)]; // Stack for flood-fill
                    let mut perimeter = 0; // Perimeter of the current region
                    let mut area: HashSet<(i32, i32)> = HashSet::new(); // Set of coordinates in the current region

                    visited[i][j] = true; // Mark the starting cell as visited

                    // Flood-fill loop
                    while let Some((x, y)) = stack.pop() {
                        area.insert((x as i32, y as i32)); // Add the current cell to the region

                        // Check all four directions (up, down, left, right)
                        for (dx, dy) in Self::DIR {
                            let nx = x as i32 + dx; // Next cell's row
                            let ny = y as i32 + dy; // Next cell's column

                            // Check if the next cell is out of bounds
                            if nx < 0 || nx >= rows as i32 || ny < 0 || ny >= cols as i32 {
                                perimeter += 1; // Add to perimeter for boundary edges
                            } else if grid[nx as usize][ny as usize] != grid[x][y] {
                                perimeter += 1; // Add to perimeter for different regions
                            } else if !visited[nx as usize][ny as usize] {
                                // Add neighboring unvisited cells to the stack
                                visited[nx as usize][ny as usize] = true;
                                stack.push((nx as usize, ny as usize));
                            }
                        }
                    }

                    // Calculate the number of sides for the current region
                    let sides = Self::count_region_sides(&area);

                    // Calculate costs for the current region
                    total_price_area += area.len() as u32 * perimeter; // Cost based on perimeter
                    total_price_sides += area.len() as u32 * sides; // Cost based on sides
                }
            }
        }

        (total_price_area, total_price_sides)
    }

    /// Counts the total number of sides required to enclose a region.
    ///
    /// # Arguments
    /// - `region`: A set of coordinates representing the region.
    ///
    /// # Returns
    /// - The total number of sides for the region.
    fn count_region_sides(region: &HashSet<(i32, i32)>) -> u32 {
        let mut side_count = 0u32; // Counter for sides

        for dir in Self::DIR {
            // Check each direction for sides
            let mut sides: HashSet<(i32, i32)> = HashSet::new();

            for pos in region {
                let tmp = (pos.0 + dir.0, pos.1 + dir.1);
                if !region.contains(&tmp) {
                    sides.insert(tmp); // Add boundary sides
                }
            }

            // Eliminate straight-line overlaps
            let mut remove: HashSet<(i32, i32)> = HashSet::new();
            for side in &sides {
                let mut tmp = (side.0 + dir.1, side.1 + dir.0); // Check the next cell along the direction
                while sides.contains(&tmp) {
                    remove.insert(tmp);
                    tmp = (tmp.0 + dir.1, tmp.1 + dir.0);
                }
            }
            side_count += (sides.len() - remove.len()) as u32; // Count only unique sides
        }

        side_count
    }

    /// Solves Part 1 of the puzzle: Calculates the total price based on area and perimeter.
    pub fn part_1(&self, input: &str) -> String {
        let map = Self::parse_input(&input); // Parse the input into a 2D grid
        Self::calculate_price(&map).0.to_string() // Get the area-based price and convert to a string
    }

    /// Solves Part 2 of the puzzle: Calculates the total price based on area and sides.
    pub fn part_2(&self, input: &str) -> String {
        let map = Self::parse_input(&input); // Parse the input into a 2D grid
        Self::calculate_price(&map).1.to_string() // Get the side-based price and convert to a string
    }
}

mod test {
    #[test]
    /// Test for Part 1: Verifies correct area-based price calculation.
    fn test_aoc2024_day12_part_1() {
        use crate::solutions::aoc2024::Day12;
        let day12 = Day12 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
        "#;
        assert_eq!(day12.part_1(input), "1930"); // Expected result for Part 1
    }

    #[test]
    /// Test for Part 2: Verifies correct side-based price calculation.
    fn test_aoc2024_day12_part_2() {
        use crate::solutions::aoc2024::Day12;
        let day12 = Day12 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
        "#;
        assert_eq!(day12.part_2(input), "1206"); // Expected result for Part 2
    }
}
