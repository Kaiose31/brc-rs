use memchr::memchr;
use memmap::MmapOptions;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::str;
use std::thread;

use crate::values::{self, Values};

pub fn solve(path: &str) {
    let cores = thread::available_parallelism().unwrap().into();
    let f = File::open(path).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&f).unwrap() };
    let mut map: HashMap<String, Values> = HashMap::with_capacity(10_000);
    let chunk_size = mmap.len() / cores;
    let mut chunks: Vec<&str> = Vec::with_capacity(cores);
    let mut offset = 0;

    for _ in 0..cores {
        let mut last = mmap.len().min(offset + chunk_size);
        last += memchr(10, &mmap[last..]).unwrap_or_default();

        let s = unsafe { str::from_utf8_unchecked(&mmap[offset..last]) };
        chunks.push(s);
        offset = last;
    }

    let res: Vec<HashMap<String, Values>> = chunks.par_iter().map(|s| process_chunks(s)).collect();

    for m in &res {
        merge_map(&mut map, m);
    }

    values::write_output(map);
}

fn merge_map(current: &mut HashMap<String, Values>, other: &HashMap<String, Values>) {
    for (key, value) in other {
        current
            .entry(key.to_string())
            .and_modify(|v| v.combine(value))
            .or_insert(*value);
    }
}

fn process_chunks(s: &str) -> HashMap<String, Values> {
    let mut map: HashMap<String, Values> = HashMap::with_capacity(10_000);

    for line in s.lines() {
        if let Some((station, temp)) = line.split_once(';') {
            let tmp = temp.parse::<f32>().unwrap();

            map.entry(station.to_string())
                .and_modify(|v| v.push(tmp))
                .or_insert_with(|| {
                    let mut values = Values::default();
                    values.push(tmp);
                    values
                });
        }
    }
    map
}
