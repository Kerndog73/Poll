use super::SessionID;

pub const OPTION_LENGTH: usize = 64;
pub const OPTION_COUNT: usize = 16;

#[derive(Debug)]
pub struct PollCat {
    pub owner: SessionID,
    pub title: String,
    pub mutex: bool,
    pub options: Vec<String>,
}
