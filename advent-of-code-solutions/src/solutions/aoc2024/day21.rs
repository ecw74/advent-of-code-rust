use crate::AoCSolution;

pub struct Day21 {
    pub day: u32,
    pub year: u32,
    pub desc: String,
    pub code: String,
}

impl Day21 {
    pub fn new(day: u32, year: u32, desc: &str, code: &[u8]) -> Self {
        Day21 {
            day,
            year,
            desc: desc.to_string(),
            code: String::from_utf8_lossy(code).to_string(),
        }
    }
}

impl AoCSolution for Day21 {
    fn day(&self) -> u32 {
        self.day
    }
    fn year(&self) -> u32 {
        self.year
    }
    fn desc(&self) -> String {
        self.desc.clone()
    }
    fn code(&self) -> String {
        self.code.clone()
    }

    fn factory(&self) -> Box<dyn AoCSolution> {
        Box::new(Self {
            day: self.day,
            year: self.year,
            desc: self.desc.clone(),
            code: self.code.clone(),
        })
    }

    fn part_1_final(&self, input: &str) -> String {
        self.part_1(input)
    }

    fn part_2_final(&self, input: &str) -> String {
        self.part_2(input)
    }
}
