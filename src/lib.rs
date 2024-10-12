mod datetime;
mod file;
mod task;

use std::fmt::Display;

use crate::task::Task;

pub enum Command {
    Add(String),
    Update(u32, String),
    Delete(u32),
    Mark(Status, u32),
    List(Option<Status>),
    None,
}

#[derive(Clone, Default, PartialEq)]
pub enum Status {
    #[default]
    Todo,
    InProgress,
    Done,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Todo => write!(f, "todo"),
            Self::InProgress => write!(f, "in-progress"),
            Self::Done => write!(f, "done"),
        }
    }
}

impl Status {
    pub fn from(status: &str) -> Self {
        match status {
            "todo" => Status::Todo,
            "in-progress" => Status::InProgress,
            "done" => Status::Done,
            &_ => todo!(),
        }
    }
}
