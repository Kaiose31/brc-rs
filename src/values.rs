use std::fmt::Display;

#[derive(Debug)]
pub struct Values {
    pub min: f32,
    pub max: f32,
    pub sum: f32,
    pub count: u16,
}

impl Values {
    pub fn new() -> Self {
        Values {
            min: 100.0,
            max: -100.0,
            sum: 0.0,
            count: 0,
        }
    }

    pub fn push(&mut self, val: f32) {
        self.min = self.min.min(val);
        self.max = self.max.max(val);
        self.count += 1;
        self.sum += val;
    }
}

impl Display for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{};{};{}",
            self.min,
            self.sum / self.count as f32,
            self.max
        )
    }
}
