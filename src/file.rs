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
