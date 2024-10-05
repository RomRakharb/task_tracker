use std::env;

mod datetime;
mod file;
mod task;

use crate::datetime::DateTime;
use crate::file::{read_file, write_file};
use crate::task::{Status, Task};

enum Command {
    Add(String),
    Update(u32, String),
    Delete(u32),
    Mark(Status, u32),
    List(Option<Status>),
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    println!("{:?}", DateTime::now());
}
