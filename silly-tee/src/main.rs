
use std::{
    fs::File,
    io::{Read},
    os::unix::io::FromRawFd
};

fn main() {
    println!("got here");
    let mut f = unsafe { File::from_raw_fd(0) };
    println!("and here");
    let mut input = String::new();
    let result = f.read_to_string(&mut input);
    println!("not here");
    println!("{:?}", result);

    println!("read: {} -- {} {:?}", input, input.len(), result);
    println!("hey there {:?}", input.len());
}
