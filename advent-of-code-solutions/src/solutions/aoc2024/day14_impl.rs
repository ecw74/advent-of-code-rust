use super::day14::Day14;
use num::integer::lcm;
use std::collections::HashSet;

/// Represents a robot with its position and velocity.
#[derive(Debug)]
struct Robot {
    position: (i32, i32), // The current position of the robot (x, y).
    velocity: (i32, i32), // The velocity of the robot (vx, vy).
}

impl Day14 {
    /// Parses the input string into a vector of `Robot` structs.
    fn parse_robot_data(input: &str) -> Vec<Robot> {
        input
            .lines() // Split the input into lines.
            .filter(|line| !line.trim().is_empty()) // Ignore empty lines.
            .filter_map(|line| {
                // Attempt to parse each line into a `Robot`.
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() == 2 {
                    // Ensure the line has exactly two parts: position and velocity.
                    let p = parts[0].strip_prefix("p=")?; // Extract the position part.
                    let v = parts[1].strip_prefix("v=")?; // Extract the velocity part.
                    let position: Vec<i32> = p
                        .split(',') // Split the position into x and y components.
                        .filter_map(|num| num.parse::<i32>().ok()) // Parse each component as an integer.
                        .collect();
                    let velocity: Vec<i32> = v
                        .split(',') // Split the velocity into x and y components.
                        .filter_map(|num| num.parse::<i32>().ok()) // Parse each component as an integer.
                        .collect();

                    // Ensure both position and velocity have exactly two components.
                    if position.len() == 2 && velocity.len() == 2 {
                        return Some(Robot {
                            position: (position[0], position[1]),
                            velocity: (velocity[0], velocity[1]),
                        });
                    }
                }
                None // Return `None` if parsing fails.
            })
            .collect() // Collect all successfully parsed robots into a vector.
    }

    /// Updates the positions of all robots after `t` seconds, considering field wrapping.
    fn update_positions(robots: &mut Vec<Robot>, field: (i32, i32), t: i32) {
        for robot in robots.iter_mut() {
            let (x0, y0) = robot.position; // Initial position.
            let (vx, vy) = robot.velocity; // Velocity.

            // Calculate new position based on velocity and time.
            let new_x = x0 + vx * t;
            let new_y = y0 + vy * t;

            // Wrap the position within the field dimensions.
            let wrapped_x = ((new_x % field.0) + field.0) % field.0;
            let wrapped_y = ((new_y % field.1) + field.1) % field.1;

            // Update the robot's position.
            robot.position = (wrapped_x, wrapped_y);
        }
    }

    /// Counts the number of robots in each of the four quadrants.
    /// Robots on the midpoint lines are excluded.
    fn count_robots_in_quadrants(
        robots: &mut Vec<Robot>,
        field: (i32, i32),
    ) -> (usize, usize, usize, usize) {
        let x_mid = field.0 / 2; // Horizontal midpoint.
        let y_mid = field.1 / 2; // Vertical midpoint.

        let mut top_left = 0;
        let mut top_right = 0;
        let mut bottom_left = 0;
        let mut bottom_right = 0;

        for robot in robots {
            let (x, y) = robot.position;

            // Exclude robots exactly on the midpoint lines.
            if x == x_mid || y == y_mid {
                continue;
            }

            // Determine the quadrant of the robot.
            if x < x_mid && y < y_mid {
                top_left += 1;
            } else if x > x_mid && y < y_mid {
                top_right += 1;
            } else if x < x_mid && y > y_mid {
                bottom_left += 1;
            } else if x > x_mid && y > y_mid {
                bottom_right += 1;
            }
        }

        (top_left, top_right, bottom_left, bottom_right)
    }

    /// Solves part 1 of the puzzle: calculates the safety factor after 100 seconds.
    pub fn part_1(&self, input: &str) -> String {
        let mut robots = Self::parse_robot_data(input); // Parse the input into robots.
        let field;

        // Determine the field dimensions based on input size (test or actual puzzle).
        match robots.len() {
            12 => field = (11, 7),   // Test field dimensions.
            _ => field = (101, 103), // Puzzle field dimensions.
        }

        Self::update_positions(&mut robots, field, 100); // Update positions after 100 seconds.
        let positions = Self::count_robots_in_quadrants(&mut robots, field); // Count robots in quadrants.

        // Calculate the safety factor and return it as a string.
        (positions.0 * positions.1 * positions.2 * positions.3).to_string()
    }

    /// Checks if all robot positions are unique (used for part 2).
    fn all_positions_unique(robots: &[Robot]) -> bool {
        let mut positions = HashSet::new();
        for r in robots {
            // If a position is already in the set, return false.
            if !positions.insert(r.position) {
                return false;
            }
        }
        true
    }

    /// Solves part 2 of the puzzle: finds the fewest seconds for all robots to form a unique arrangement.
    pub fn part_2(&self, input: &str) -> String {
        let mut robots = Self::parse_robot_data(input); // Parse the input into robots.
        let field;

        // Determine the field dimensions based on input size (test or actual puzzle).
        match robots.len() {
            12 => field = (11, 7),   // Test field dimensions.
            _ => field = (101, 103), // Puzzle field dimensions.
        }

        let mut final_time = 0;

        // Simulate until all positions are unique.
        // since the robots wrap at the borders around I think after lcm
        // the robots on there starting position again
        for time in 1..lcm(field.0, field.1) + 1 {
            Self::update_positions(&mut robots, field, 1); // Incrementally update positions.
            // I expect the Christmas tree is there when all robots in a unique position
            if Self::all_positions_unique(&mut robots) {
                final_time = time; // Record the time when all positions are unique.
                break;
            }
        }

        final_time.to_string() // Return the time as a string.
    }
}

mod test {
    #[test]
    /// Test for part 1 of Day 14: validates the safety factor calculation.
    fn test_aoc2024_day14_part_1() {
        use crate::solutions::aoc2024::Day14;
        let day14 = Day14 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
        "#;
        assert_eq!(day14.part_1(input), "12"); // Ensure the result matches the expected value.
    }

    #[test]
    /// Test for part 2 of Day 14: validates the unique arrangement time.
    fn test_aoc2024_day14_part_2() {
        use crate::solutions::aoc2024::Day14;
        let day14 = Day14 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
        "#;
        assert_eq!(day14.part_2(input), "1"); // Ensure the result matches the expected value.
    }
}
