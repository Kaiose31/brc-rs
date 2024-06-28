use crate::values::Values;
use std::collections::BTreeMap;
use std::fs::{read_to_string, File};
use std::io::Write;

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
        writeln!(
            file,
            "{};{};{};{}",
            station,
            vals.min,
            vals.sum / vals.count as f32,
            vals.max
        )
        .unwrap();
    }
}
