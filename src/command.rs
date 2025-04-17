#[derive(Debug)]
pub enum Command {
    Get(String),
    Set(String, String),
    Delete(String),
    Unknown,
}

// SET x 10
// GET x
// DELETE x
impl Command {
    pub fn parse(input: &str) -> Command {
        let parts: Vec<&str> = input.trim().splitn(3, ' ').collect();

        match parts.as_slice()  {
            ["GET", key] => Command::Get(key.to_string()),
            ["SET", key, value] => Command::Set(key.to_string(), value.to_string()),
            ["DELETE", key] => Command::Delete(key.to_string()),
            _ => Command::Unknown,
        }
    }
}