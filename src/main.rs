use std::{env, num::ParseIntError};

use task_cli::file::{read_file, write_file};
use task_cli::task::Tasks;
use task_cli::{Command, Status};

fn main() -> Result<(), ParseIntError> {
    let mut args = env::args();
    let command = match args.nth(1).unwrap_or("".to_string()).as_str() {
        "add" => match args.nth(2) {
            Some(description) => Command::Add(description),
            None => Command::None,
        },
        "update" => match (args.nth(2), args.nth(3)) {
            (None, _) | (_, None) => Command::None,
            (Some(id), Some(description)) => Command::Update(id.parse()?, description),
        },
        "delete" => match args.nth(2) {
            Some(id) => Command::Delete(id.parse()?),
            None => Command::None,
        },
        "mark-todo" => match args.nth(2) {
            Some(id) => Command::Mark(Status::Todo, id.parse()?),
            None => Command::None,
        },
        "mark-in-progress" => match args.nth(2) {
            Some(id) => Command::Mark(Status::InProgress, id.parse()?),
            None => Command::None,
        },
        "mark-done" => match args.nth(2) {
            Some(id) => Command::Mark(Status::Done, id.parse()?),
            None => Command::None,
        },
        "list" => match args.nth(2) {
            Some(status) => Command::List(Some(Status::from(&status))),
            None => Command::List(None),
        },
        &_ => Command::None,
    };

    let contents = read_file("task.json").unwrap();
    let mut tasks = Tasks::from(contents);
    tasks.process(command);
    let _ = write_file("task.json", todo!());
    Ok(())
}
