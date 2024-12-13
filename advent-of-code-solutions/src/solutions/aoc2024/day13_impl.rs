use super::day13::Day13;

impl Day13 {
    /// Parses the input string into the machines format.
    /// Each machine is represented as ((dx_a, dy_a), (dx_b, dy_b), (px, py)).
    /// - dx_a, dy_a: Movement caused by pressing button A.
    /// - dx_b, dy_b: Movement caused by pressing button B.
    /// - px, py: Coordinates of the prize.
    fn parse_input(input: &str) -> Vec<((i32, i32), (i32, i32), (i64, i64))> {
        let mut machines = Vec::new();

        // Split the input into blocks for each machine
        for block in input.split("\n\n") {
            // Initialize variables to store parsed values
            let mut dx_a = 0;
            let mut dy_a = 0;
            let mut dx_b = 0;
            let mut dy_b = 0;
            let mut px = 0;
            let mut py = 0;

            // Parse each line within a block
            for line in block.lines() {
                if let Some(rest) = line.strip_prefix("Button A: x+") {
                    // Parse Button A's movement
                    let parts: Vec<&str> = rest.split(", y+").collect();
                    if parts.len() == 2 {
                        dx_a = parts[0].parse::<i32>().unwrap();
                        dy_a = parts[1].parse::<i32>().unwrap();
                    }
                } else if let Some(rest) = line.strip_prefix("Button B: x+") {
                    // Parse Button B's movement
                    let parts: Vec<&str> = rest.split(", y+").collect();
                    if parts.len() == 2 {
                        dx_b = parts[0].parse::<i32>().unwrap();
                        dy_b = parts[1].parse::<i32>().unwrap();
                    }
                } else if let Some(rest) = line.strip_prefix("Prize: x=") {
                    // Parse Prize's coordinates
                    let parts: Vec<&str> = rest.split(", y=").collect();
                    if parts.len() == 2 {
                        px = parts[0].parse::<i64>().unwrap();
                        py = parts[1].parse::<i64>().unwrap();
                    }
                }
            }

            // Store the parsed data for this machine
            machines.push(((dx_a, dy_a), (dx_b, dy_b), (px, py)));
        }

        machines
    }

    /// Solves the system of linear equations for a single claw machine.
    /// Determines the number of presses for buttons A and B required to reach the prize.
    ///
    /// This method solves the equations:
    ///   a * a_x + b * b_x = x
    ///   a * a_y + b * b_y = y
    /// - (a_x, a_y): Movement caused by pressing button A.
    /// - (b_x, b_y): Movement caused by pressing button B.
    /// - (x, y): Prize coordinates.
    ///
    /// Returns (a, b) if a valid solution exists within the range [0, 100].
    /// Returns (0, 0) otherwise.
    fn solve(x: i64, y: i64, a_x: i32, a_y: i32, b_x: i32, b_y: i32) -> (i64, i64) {
        // Calculate the determinant of the system
        let denom = a_x as i64 * b_y as i64 - a_y as i64 * b_x as i64;
        let num_a = x * b_y as i64 - y * b_x as i64;
        let num_b = y * a_x as i64 - x * a_y as i64;

        // If determinant is zero, the system has no solution
        if denom == 0 {
            return (0, 0);
        }

        // Check if the solution is an integer
        if num_b % denom != 0 || num_a % denom != 0 {
            return (0, 0);
        }

        let a = num_a / denom;
        let b = num_b / denom;

        // Ensure solution is within bounds
        if a < 0 || b < 0 {
            return (0, 0);
        }

        (a, b)
    }

    /// Solves Part 1 of the puzzle.
    /// Calculates the minimum total cost to win as many prizes as possible
    /// for the initial prize coordinates.
    pub fn part_1(&self, input: &str) -> String {
        let machines = Self::parse_input(input);
        let mut total_cost = 0;

        for (dx_a, dy_a, dx_b, dy_b, px, py) in machines
            .iter()
            .map(|(a, b, c)| (a.0, a.1, b.0, b.1, c.0, c.1))
        {
            let (a, b) = Self::solve(px, py, dx_a, dy_a, dx_b, dy_b);
            // Add the cost if the solution is valid
            if a < 100 && b < 100 {
                total_cost += 3 * a + b;
            }
        }

        total_cost.to_string()
    }

    /// Solves Part 2 of the puzzle.
    /// Calculates the minimum total cost to win as many prizes as possible
    /// after adjusting the prize coordinates by adding 10^13 to both x and y.
    pub fn part_2(&self, input: &str) -> String {
        let machines = Self::parse_input(input);
        let mut total_cost = 0i64;

        for (dx_a, dy_a, dx_b, dy_b, mut px, mut py) in machines
            .iter()
            .map(|(a, b, c)| (a.0, a.1, b.0, b.1, c.0, c.1))
        {
            // Adjust prize coordinates
            px += 10000000000000;
            py += 10000000000000;

            let (a, b) = Self::solve(px, py, dx_a, dy_a, dx_b, dy_b);
            // Add the cost if the solution is valid
            total_cost += 3 * a + b;
        }

        total_cost.to_string()
    }
}

mod test {
    #[test]
    /// Tests Part 1 of Day 13 for AoC 2024.
    fn test_aoc2024_day13_part_1() {
        use crate::solutions::aoc2024::Day13;
        let day13 = Day13 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
Button A: x+94, y+34
Button B: x+22, y+67
Prize: x=8400, y=5400

Button A: x+26, y+66
Button B: x+67, y+21
Prize: x=12748, y=12176

Button A: x+17, y+86
Button B: x+84, y+37
Prize: x=7870, y=6450

Button A: x+69, y+23
Button B: x+27, y+71
Prize: x=18641, y=10279
        "#;
        assert_eq!(day13.part_1(input), "480");
    }

    #[test]
    /// Tests Part 2 of Day 13 for AoC 2024.
    fn test_aoc2024_day13_part_2() {
        use crate::solutions::aoc2024::Day13;
        let day13 = Day13 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
Button A: x+94, y+34
Button B: x+22, y+67
Prize: x=8400, y=5400

Button A: x+26, y+66
Button B: x+67, y+21
Prize: x=12748, y=12176

Button A: x+17, y+86
Button B: x+84, y+37
Prize: x=7870, y=6450

Button A: x+69, y+23
Button B: x+27, y+71
Prize: x=18641, y=10279
        "#;
        assert_eq!(day13.part_2(input), "875318608908");
    }
}
