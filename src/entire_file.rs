use std::{
    collections::BTreeMap,
    fs::File,
    io::Write,
    io::{BufReader, Read},
    str::from_utf8_unchecked,
};

use crate::values::Values;

const FILE_SIZE_BYTES: usize = 1_000_000_000 * (100 + (10 * 4) + 4 + 4);

pub fn solve(path: &str) {
    let mut buf: Vec<u8> = Vec::with_capacity(FILE_SIZE_BYTES);
    let f = File::open(path).unwrap();
    let mut reader = BufReader::new(f);
    let _ = reader.read_to_end(&mut buf).unwrap();
    let all_data: &str = unsafe { from_utf8_unchecked(&buf) };
    let map = parse_data(all_data);

    //Write output
    let mut file = File::create("data/output.txt").unwrap();
    for (station, vals) in &map {
        writeln!(
            file,
            "{};{:.1};{:.1};{:.1}",
            station,
            vals.min,
            vals.sum / vals.count as f32,
            vals.max
        )
        .unwrap();
    }
}

fn parse_data(s: &str) -> BTreeMap<String, Values> {
    let mut map: BTreeMap<String, Values> = BTreeMap::new();
    let mut cur_val = String::with_capacity(100 + 5 * 4);
    let mut station = String::with_capacity(100);
    for c in s.chars() {
        match c {
            ';' => {
                station.clone_from(&cur_val);
                cur_val.clear();
            }
            '\n' => {
                map.entry(station.clone())
                    .or_insert(Values::default())
                    .push(cur_val.parse::<f32>().unwrap());
                cur_val.clear();
            }
            _ => cur_val.push(c),
        }
    }
    map
}
