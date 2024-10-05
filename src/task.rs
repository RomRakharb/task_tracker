use crate::datetime::DateTime;

#[derive(Default, Debug)]
enum Status {
    #[default]
    Todo,
    InProgress,
    Done,
}

#[derive(Default, Debug)]
pub struct Task {
    id: u32,
    description: String,
    status: Status,
    created_at: DateTime,
    updated_at: DateTime,
}

#[cfg(test)]
mod tests {
    use super::*;
}
