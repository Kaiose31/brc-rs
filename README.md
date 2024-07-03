# Increasingly performant implementations of 1BRC

1. Naive (read_lines with BTreeMap, Single Threaded) 

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/brc_rs` | 223.916 ± 2.057 | 221.079 | 227.251 | 1.00 |

2. Read Entire File(Single, Threaded)

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/brc_rs` | 131.652 ± 0.797 | 131.149 | 133.036 | 1.00 |

3. Buffered Read and Write (With Hashmap, Single Threaded)

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/brc_rs` | 97.004 ± 0.457 | 96.412 | 97.717 | 1.00 |
