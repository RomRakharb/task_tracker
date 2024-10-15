use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum Command {
    Add(String),
    Update(u32, String),
    Delete(u32),
    Mark(Status, u32),
    List(Option<Status>),
    None,
}

#[derive(Clone, Default, Debug, PartialEq)]
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

pub mod task {
    use crate::datetime::DateTime;
    use crate::file::write_file;
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
        pub fn from_contents(contents: String) -> Self {
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
            Self(tasks)
        }

        pub fn to_contents(&self) -> String {
            let mut contents = String::new();
            contents.push_str("[\n");
            for task in self.0.clone() {
                contents.push_str("  {\n");
                contents.push_str(format!("    \"id\" : {},\n", task.id).as_str());
                contents.push_str(
                    format!("    \"description\" : \"{}\",\n", task.description).as_str(),
                );
                contents.push_str(format!("    \"status\" : \"{}\",\n", task.status).as_str());
                contents.push_str(
                    format!(
                        "    \"createdAt\" : \"{}\",\n",
                        task.created_at.to_iso8601()
                    )
                    .as_str(),
                );
                contents.push_str(
                    format!("    \"updatedAt\" : \"{}\"\n", task.updated_at.to_iso8601()).as_str(),
                );
                contents.push_str("  },\n");
            }
            contents.pop();
            contents.pop();
            contents.push_str("\n]");
            contents
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
                    task.updated_at = DateTime::now();
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
                    task.updated_at = DateTime::now();
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
                            "ID: {} description: {} status: {} created at: {} updated at: {}",
                            task.id,
                            task.description,
                            task.status,
                            task.created_at.to_iso8601(),
                            task.updated_at.to_iso8601()
                        );
                    }
                }
                None => {
                    for task in self.0.iter() {
                        println!(
                            "ID: {} description: {} status: {} created at: {} updated at: {}",
                            task.id,
                            task.description,
                            task.status,
                            task.created_at.to_iso8601(),
                            task.updated_at.to_iso8601()
                        );
                    }
                }
            }
        }

        pub fn process(&mut self, command: Command) {
            match command {
                Command::Add(description) => self.add(description),
                Command::Update(id, description) => self.update(id, description),
                Command::Delete(id) => self.delete(id),
                Command::Mark(status, id) => self.mark(status, id),
                Command::List(status) => self.list(status),
                Command::None => {}
            }
            let _ = write_file("tasks.json", self.to_contents());
        }
    }
}

pub mod datetime {
    use std::{num::ParseIntError, time::SystemTime};

    #[derive(Clone, Default, Debug)]
    pub struct DateTime {
        years: u16,
        months: u8,
        days: u8,
        hours: u8,
        minutes: u8,
        seconds: u8,
    }

    impl DateTime {
        pub fn now() -> Self {
            const SECONDS_IN_MINUTE: u64 = 60;
            const SECONDS_IN_HOUR: u64 = 60 * SECONDS_IN_MINUTE;
            const SECONDS_IN_DAY: u64 = 24 * SECONDS_IN_HOUR;

            const MONTHS_IN_YEAR: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            const MONTHS_IN_LEAP_YEAR: [u64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

            const SECOND_IN_YEAR: u64 = 365 * SECONDS_IN_DAY;
            const SECOND_IN_LEAP_YEAR: u64 = 366 * SECONDS_IN_DAY;

            let mut time_pool: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
            {
                Ok(time) => time.as_secs(),
                Err(e) => {
                    eprintln!("Error at DateTime::now : {e}");
                    return Self::default();
                }
            };

            let mut years = 1970;
            let mut months = 1;
            let mut days = 1;
            let mut hours = 0;
            let mut minutes = 0;

            while time_pool > SECOND_IN_YEAR {
                if is_leap_year(years) {
                    time_pool -= SECOND_IN_LEAP_YEAR;
                } else {
                    time_pool -= SECOND_IN_YEAR;
                }
                years += 1;
            }

            let this_year = if is_leap_year(years) {
                MONTHS_IN_LEAP_YEAR
            } else {
                MONTHS_IN_YEAR
            };

            for month in this_year {
                if time_pool < month * SECONDS_IN_DAY {
                    break;
                }
                time_pool -= month * SECONDS_IN_DAY;
                months += 1;
            }

            days += time_pool / SECONDS_IN_DAY;
            time_pool %= SECONDS_IN_DAY;

            hours += time_pool / SECONDS_IN_HOUR;
            time_pool %= SECONDS_IN_HOUR;

            minutes += time_pool / SECONDS_IN_MINUTE;
            time_pool %= SECONDS_IN_MINUTE;

            Self {
                years,
                months: months as u8,
                days: days as u8,
                hours: hours as u8,
                minutes: minutes as u8,
                seconds: time_pool as u8,
            }
        }

        pub fn from_iso8601(iso8601: &str) -> Result<Self, ParseIntError> {
            let datetime = Self {
                years: iso8601[0..=3].parse()?,
                months: iso8601[5..=6].parse()?,
                days: iso8601[8..=9].parse()?,
                hours: iso8601[11..=12].parse()?,
                minutes: iso8601[14..=15].parse()?,
                seconds: iso8601[17..=18].parse()?,
            };
            Ok(datetime)
        }

        pub fn to_iso8601(&self) -> String {
            format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                self.years, self.months, self.days, self.hours, self.minutes, self.seconds
            )
        }
    }

    fn is_leap_year(year: u16) -> bool {
        (year - 1968) % 4 == 0
    }
}

pub mod file {
    use std::fs::File;
    use std::io::prelude::*;

    pub fn read_file(file_name: &str) -> std::io::Result<String> {
        let mut file = File::open(file_name)
            .or_else(|_| File::create(file_name).and_then(|_| File::open(file_name)))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn write_file(file_name: &str, contents: String) -> std::io::Result<()> {
        let mut file = File::create(file_name)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use std::fs;

        use super::*;

        #[test]
        fn test_read_file() {
            assert_eq!(read_file("test.json").unwrap(), String::new());
            let _ = fs::remove_file("test.json");
        }

        #[test]
        fn test_write_file() {
            let _ = write_file("test.json", String::from("test"));
            assert_eq!(read_file("test.json").unwrap(), String::from("test"));
            let _ = fs::remove_file("test.json");
        }
    }
}
