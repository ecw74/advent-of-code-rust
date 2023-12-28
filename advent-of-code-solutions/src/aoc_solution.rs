pub trait AoCSolution: Send + Sync {
    fn day(&self) -> u32;
    fn year(&self) -> u32;
    fn desc(&self) -> String;
    fn code(&self) -> String;
    fn part_1_final(&self, input: &str) -> i64;
    fn part_2_final(&self, input: &str) -> i64;
    fn factory(&self) -> Box<dyn AoCSolution>;
}