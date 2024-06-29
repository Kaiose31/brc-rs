mod buffer;
mod entire_file;
mod naive;
mod values;
fn main() {
    naive::solve("data/measurements.txt");
    buffer::solve("data/measurements.txt");
    entire_file::solve("data/measurements.txt");
}
