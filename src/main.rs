use std::env;

use task_cli;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
