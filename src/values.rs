use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufWriter, Write},
};

#[derive(Debug, Clone, Copy)]
pub struct Values {
    pub min: f32,
    pub max: f32,
    pub sum: f32,
    pub count: u32,
}

impl Values {
    pub fn push(&mut self, val: f32) {
        self.min = self.min.min(val);
        self.max = self.max.max(val);
        self.count += 1;
        self.sum += val;
    }

    pub fn combine(&mut self, other: &Self) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self.count += other.count;
        self.sum += other.sum;
    }
}

impl Default for Values {
    fn default() -> Self {
        Values {
            min: 100.0,
            max: -100.0,
            sum: 0.0,
            count: 0,
        }
    }
}

impl Display for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{};{:.1};{}",
            self.min,
            self.sum / self.count as f32,
            self.max
        )
    }
}

pub fn write_output(map: HashMap<String, Values>) {
    let file = File::create("data/output.txt").unwrap();

    let mut writer = BufWriter::new(file);

    let mut res: Vec<String> = map
        .into_iter()
        .map(|(a, b)| a + ";" + &b.to_string())
        .collect();
    res.sort_unstable();

    for line in &res {
        writer.write_all(line.as_bytes()).unwrap();
        writer.write_all(b"\n").unwrap();
    }
}
