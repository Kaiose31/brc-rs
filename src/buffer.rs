use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

use crate::values::{self, Values};

pub fn solve(path: &str) {
    let file = File::open(path).expect("Unable to open file");
    let mut map: HashMap<String, Values> = HashMap::with_capacity(10_000);
    let reader = BufReader::new(file);
    let binding = reader.lines();
    for line in binding.map_while(Result::ok) {
        if let Some((station, temp)) = line.split_once(';') {
            build_map(station.to_string(), temp, &mut map);
        }
    }

    values::write_output(map);
}

fn build_map(s: String, t: &str, map: &mut HashMap<String, Values>) {
    let tmp = t.parse::<f32>().unwrap();

    map.entry(s).and_modify(|v| v.push(tmp)).or_insert_with(|| {
        let mut values = Values::default();
        values.push(tmp);
        values
    });
}
