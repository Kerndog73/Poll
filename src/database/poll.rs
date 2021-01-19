pub const POLL_ID_LENGTH: usize = 8;
pub type PollID = String;

pub const TITLE_LENGTH: usize = 128;

macro_rules! poll_duration {
    () => { "INTERVAL '1 day'" }
}
