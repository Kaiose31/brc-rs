use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek};
use std::path::PathBuf;
use std::thread::{self, JoinHandle};

use crate::values::Values;
pub fn solve(path: &str) {
    read_file(PathBuf::from(path));
}

const BUFFER_SIZE: usize = 4096 * 4096 * 2;

trait ReadTill {
    fn read_till(&mut self, byte: u8) -> (String, usize);
}

impl ReadTill for File {
    fn read_till(&mut self, byte: u8) -> (String, usize) {
        //buffer length could be dynamic
        let mut buf = [0_u8; 105];
        let nbytes = self.read(&mut buf).unwrap();
        let mut last = 0;
        for (i, v) in buf[..nbytes].iter().enumerate() {
            if *v == byte {
                last = i;
                break;
            }
        }
        (
            unsafe { String::from_utf8_unchecked(buf[..last].to_vec()) },
            last,
        )
    }
}

fn read_file(path: PathBuf) {
    let mut f = File::open(path).unwrap();
    let mut buf = vec![0_u8; BUFFER_SIZE];
    let mut map: HashMap<String, Values> = HashMap::new();
    let mut offset = 0;
    let mut t_handles: Vec<JoinHandle<HashMap<String, Values>>> = Vec::new();
    let mut i = 0;
    loop {
        let nbytes = f.read(&mut buf).unwrap();
        //Reached EOF
        offset += nbytes;
        if nbytes == 0 {
            break;
        }
        let mut s = unsafe { String::from_utf8_unchecked(buf.to_vec()) };

        let (s2, leftover) = f.read_till(10);
        s.push_str(&s2);
        offset += leftover + 1;

        t_handles.push(thread::spawn(move || process_chunk(s)));
        dbg!("processing chunk", i);
        i += 1;
        f.seek(std::io::SeekFrom::Start(offset.try_into().unwrap()))
            .unwrap();

        if nbytes != BUFFER_SIZE {
            break;
        }
    }
    for (i, jh) in t_handles.into_iter().enumerate() {
        dbg!("merging map", i);
        merge_maps(&mut map, &jh.join().unwrap());
    }
}

fn merge_maps(current: &mut HashMap<String, Values>, other: &HashMap<String, Values>) {
    for (key, value) in other {
        current
            .entry(key.to_string())
            .and_modify(|v| v.combine(value))
            .or_insert(*value);
    }
}

fn process_chunk(s: String) -> HashMap<String, Values> {
    let mut map: HashMap<String, Values> = HashMap::with_capacity(10_000);
    for line in s.lines() {
        if let Some((station, temp)) = line.split_once(';') {
            let tmp = temp.parse::<f32>().unwrap();
            map.entry(station.to_owned())
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
