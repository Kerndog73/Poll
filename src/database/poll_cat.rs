use crate::utils;
use deadpool_postgres::{Pool, PoolError};
use super::{SessionID, PollID, POLL_ID_LENGTH};

pub const OPTION_LENGTH: usize = 64;
pub const OPTION_COUNT: usize = 16;

#[derive(Debug)]
pub struct PollCat {
    pub owner: SessionID,
    pub title: String,
    pub mutex: bool,
    pub options: Vec<String>,
}

pub async fn create_poll_cat(pool: Pool, poll: PollCat) -> Result<PollID, PoolError> {
    let conn = pool.get().await?;
    let poll_stmt = conn.prepare("
        INSERT INTO poll_categorical (poll_id, session_id, creation_time, title, mutex)
        VALUES ($1, $2, NOW(), $3, $4)
        ON CONFLICT (poll_id) DO NOTHING
    ").await?;
    let option_stmt = conn.prepare("
        INSERT INTO poll_categorical_option (poll_id, sequence, name)
        VALUES ($1, $2, $3)
    ").await?;

    let mut poll_id = utils::generate_random_base64url(POLL_ID_LENGTH);
    while conn.execute(&poll_stmt, &[&poll_id, &poll.owner, &poll.title, &poll.mutex]).await? == 0 {
        poll_id = utils::generate_random_base64url(POLL_ID_LENGTH);
    }

    for i in 0..poll.options.len() {
        conn.execute(&option_stmt, &[&poll_id, &(i as i32), &poll.options[i]]).await?;
    }

    Ok(poll_id)
}
