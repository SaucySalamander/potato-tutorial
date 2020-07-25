mod window;
mod io;
mod vulkan;

//use crate::io::file::{read_file_to_str, write_file, append_file};
use window::init;

fn main() {
    // let _ = read_file_to_str("./config.yaml".as_ref());
    // let _ = write_file("./test.txt".as_ref(), "".as_ref());
    // let _ = append_file("./test.txt".as_ref(), "this is a test".as_ref());
    init();
}
