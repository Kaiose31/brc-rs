# Increasingly performant implementations of 1BRC

1. Naive (read_lines with BTreeMap, Single Threaded) 
```
| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/brc_rs` | 223.916 ± 2.057 | 221.079 | 227.251 | 1.00 |
```