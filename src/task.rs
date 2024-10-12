use core::str;
use std::any::Any;

use crate::datetime::DateTime;

use crate::{Command, Status};

#[derive(Default, Clone)]
pub struct Task {
    id: u32,
    description: String,
    status: Status,
    created_at: DateTime,
    updated_at: DateTime,
}

pub struct Tasks(Vec<Task>);

impl Tasks {
    pub fn from(contents: String) -> Self {
        let mut tasks: Vec<Task> = Vec::new();
        let mut task = Task::default();
        for line in contents.lines() {
            match line.trim() {
                "[" | "]" => {}
                "{" => task = Task::default(),
                "}" | "}," => tasks.push(task.clone()),
                _ => {
                    let json_line: Vec<&str> = line.trim().split(" : ").collect();
                    let property = json_line[0].trim_matches('\"');
                    let value = json_line[1].trim_end_matches(',').trim_matches('\"');
                    match property {
                        "id" => task.id = value.parse().unwrap(), //TODO: error handling
                        "description" => task.description = value.to_string(),
                        "status" => {
                            task.status = match value {
                                "todo" => Status::Todo,
                                "in-progress" => Status::InProgress,
                                "done" => Status::Done,
                                _ => todo!(), //TODO: error handling
                            }
                        }
                        "createdAt" => task.created_at = DateTime::from_iso8601(value).unwrap(),
                        "updatedAt" => task.updated_at = DateTime::from_iso8601(value).unwrap(),
                        _ => {}
                    }
                }
            }
        }
        Tasks(tasks)
    }

    fn add(&mut self, description: String) {
        let last_id = match self.0.last() {
            Some(task) => task.id,
            None => 0,
        };

        let task = Task {
            id: last_id + 1,
            description,
            status: Status::Todo,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };
        self.0.push(task);
        println!("Task(ID: {}) added successfully", last_id + 1);
    }

    fn update(&mut self, id: u32, description: String) {
        match self.0.iter_mut().find(|x| x.id == id) {
            Some(task) => {
                task.description = description.clone();
                println!("Task(ID: {}) updated to: {}", id, description);
            }
            None => println!("Task(ID: {}) does not exist", id),
        };
    }

    fn delete(&mut self, id: u32) {
        match self.0.iter().position(|x| x.id == id) {
            Some(index) => {
                let _ = self.0.remove(index);
                println!("Task(ID: {}) deleted successfully", id);
            }
            None => println!("Task(ID: {}) does not exist", id),
        };
    }

    fn mark(&mut self, status: Status, id: u32) {
        match self.0.iter_mut().find(|x| x.id == id) {
            Some(task) => {
                let print_status = match status {
                    Status::Todo => "todo",
                    Status::InProgress => "in-progress",
                    Status::Done => "done",
                };
                task.status = status;
                println!("Task(ID: {}) marked as: {}", id, print_status);
            }
            None => println!("Task(ID: {}) does not exist", id),
        };
    }

    fn list(&self, status: Option<Status>) {
        match status {
            Some(status) => {
                for task in self.0.iter().filter(|x| x.status == status) {
                    println!(
                        "ID: {}\tdescription: {}\tstatus: {}\tcreated at: {}\tupdated at: {}",
                        task.id,
                        task.description,
                        task.status,
                        task.created_at.to_iso8601(),
                        task.updated_at.to_iso8601()
                    );
                }
            }
            None => {}
        }
    }

    fn process(&mut self, command: Command) {
        match command {
            Command::Add(description) => self.add(description),
            Command::Update(id, description) => self.update(id, description),
            Command::Delete(id) => self.delete(id),
            Command::Mark(status, id) => self.mark(status, id),
            Command::List(status) => self.list(status),
            Command::None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
