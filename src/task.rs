use crate::datetime::DateTime;

#[derive(Clone, Default, Debug)]
pub enum Status {
    #[default]
    Todo,
    InProgress,
    Done,
}

#[derive(Default, Debug, Clone)]
pub struct Task {
    id: u32,
    description: String,
    status: Status,
    created_at: DateTime,
    updated_at: DateTime,
}

impl Task {
    pub fn from_contents(contents: String) -> Vec<Self> {
        let mut tasks: Vec<Self> = Vec::new();
        let mut task = Self::default();
        for line in contents.lines() {
            match line.trim() {
                "[" | "]" => {}
                "{" => task = Self::default(),
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
                                _ => Status::Todo, //TODO: error handling
                            }
                        }
                        "createdAt" => task.created_at = DateTime::from_iso8601(value).unwrap(),
                        "updatedAt" => task.updated_at = DateTime::from_iso8601(value).unwrap(),
                        _ => {}
                    }
                }
            }
        }
        tasks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
