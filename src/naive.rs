use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fs::{read_to_string, File};
use std::io::Write;
#[derive(Debug)]
struct Values {
    min: f32,
    max: f32,
    mean: f32,
    count: u16,
}

impl Values {
    fn new() -> Self {
        Values {
            min: 100.0,
            max: -100.0,
            mean: 0.0,
            count: 0,
        }
    }

    fn push(&mut self, val: f32) {
        self.min = self.min.min(val);
        self.max = self.max.max(val);
        self.count += 1;
        self.mean = (val + self.mean) / self.count as f32;
    }
}

pub fn solve(path: &str) {
    let mut temps = BTreeMap::new();
    let binding = read_to_string(path).unwrap();
    for line in binding.lines() {
        if let Some((station, temp)) = line.split_once(';') {
            if !temps.contains_key(station) {
                let mut values = Values::new();
                values.push(temp.parse::<f32>().unwrap());
                temps.insert(station, values);
            } else {
                temps
                    .get_mut(station)
                    .unwrap()
                    .push(temp.parse::<f32>().unwrap());
            }
        }
    }
    //Done processing, Generate output Data
    let mut file = File::create("data/output.txt").unwrap();
    for (station, vals) in &temps {
        writeln!(file, "{};{};{};{}", station, vals.min, vals.mean, vals.max).unwrap();
    }
}
