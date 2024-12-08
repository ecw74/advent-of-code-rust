pub trait AoCSolution: Send + Sync {
    /// Returns the day of the Advent of Code challenge.
    fn day(&self) -> u32;

    /// Returns the year of the Advent of Code challenge.
    fn year(&self) -> u32;

    /// Provides a brief description of the problem.
    fn desc(&self) -> String;

    /// Returns the source code or reference to the solution.
    fn code(&self) -> String;

    /// Solves part 1 of the problem using the given input.
    fn part_1_final(&self, input: &str) -> String;

    /// Optionally returns runtime metrics for part 1.
    fn part_1_runtime(&self) -> usize;

    /// Solves part 2 of the problem using the given input.
    fn part_2_final(&self, input: &str) -> String;

    /// Optionally returns runtime metrics for part 2.
    fn part_2_runtime(&self) -> usize;

    /// Factory method to create a boxed instance of the solution.
    /// Consider replacing this with a free function or `impl`.
    fn factory(&self) -> Box<dyn AoCSolution>;
}
