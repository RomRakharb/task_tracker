use std::{env, num::ParseIntError};

use task_cli::file::read_file;
use task_cli::task::Tasks;
use task_cli::{Command, Status};

fn main() -> Result<(), ParseIntError> {
    let args: Vec<String> = env::args().skip(1).collect();
    let command = match args.get(0).map(String::as_str) {
        Some("add") => match args.get(1) {
            Some(description) => Command::Add(description.clone()),
            None => Command::None,
        },
        Some("update") => match (args.get(1), args.get(2)) {
            (None, _) | (_, None) => Command::None,
            (Some(id), Some(description)) => Command::Update(id.parse()?, description.clone()),
        },
        Some("delete") => match args.get(1) {
            Some(id) => Command::Delete(id.parse()?),
            None => Command::None,
        },
        Some("mark-todo") => match args.get(1) {
            Some(id) => Command::Mark(Status::Todo, id.parse()?),
            None => Command::None,
        },
        Some("mark-in-progress") => match args.get(1) {
            Some(id) => Command::Mark(Status::InProgress, id.parse()?),
            None => Command::None,
        },
        Some("mark-done") => match args.get(1) {
            Some(id) => Command::Mark(Status::Done, id.parse()?),
            None => Command::None,
        },
        Some("list") => match args.get(1) {
            Some(status) => Command::List(Some(Status::from(&status))),
            None => Command::List(None),
        },
        Some(_) | None => Command::None,
    };

    let contents = read_file("tasks.json").unwrap();
    let mut tasks = Tasks::from_contents(contents);
    tasks.process(command);
    Ok(())
}
