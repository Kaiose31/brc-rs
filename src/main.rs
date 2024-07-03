mod buffer;
mod chunk;
mod entire_file;
mod mmaped;
mod naive;
mod values;
fn main() {
    mmaped::solve("data/measurements.txt");
}
