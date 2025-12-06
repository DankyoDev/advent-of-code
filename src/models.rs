pub struct DayResult {
    pub part1: String,
    pub part2: String,
}

impl DayResult {
    pub fn new(p1: impl Into<String>, p2: impl Into<String>) -> Self {
        Self {
            part1: p1.into(),
            part2: p2.into(),
        }
    }
}
