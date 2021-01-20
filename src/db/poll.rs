use crate::utils;
use super::SessionID;
use deadpool_postgres::{Client, Pool, PoolError};

pub const POLL_ID_LENGTH: usize = 8;
pub type PollID = String;

pub const TITLE_LENGTH: usize = 128;

// TODO: Don't forget to set this back to 1 day
macro_rules! poll_duration {
    () => { "INTERVAL '10 day'" }
}

pub const POLL_DURATION: std::time::Duration = std::time::Duration::from_secs(24 * 60 * 60);

pub struct Poll {
    pub owner: SessionID,
    pub title: String,
}

pub struct CreatedPoll {
    pub title: String,
    pub creation_time: std::time::SystemTime,
}

pub async fn create_poll(conn: &Client, poll: Poll) -> Result<PollID, PoolError> {
    let stmt = conn.prepare("
        INSERT INTO poll (poll_id, session_id, creation_time, title)
        VALUES ($1, $2, NOW(), $3)
        ON CONFLICT (poll_id) DO NOTHING
    ").await?;

    let mut poll_id = utils::generate_random_base64url(POLL_ID_LENGTH);
    while conn.execute(&stmt, &[&poll_id, &poll.owner, &poll.title]).await? == 0 {
        poll_id = utils::generate_random_base64url(POLL_ID_LENGTH);
    }

    Ok(poll_id)
}

pub async fn valid_poll_id(pool: Pool, poll_id: &PollID, session_id: &SessionID) -> Result<bool, PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare(concat!("
        SELECT 1
        FROM poll
        WHERE poll_id = $1
        AND session_id = $2
        AND creation_time > NOW() - ", poll_duration!()
    )).await?;
    Ok(conn.query_opt(&stmt, &[poll_id, session_id]).await?.is_some())
}

pub async fn get_poll(pool: Pool, poll_id: &PollID, session_id: &SessionID)
    -> Result<Option<CreatedPoll>, PoolError>
{
    let conn = pool.get().await?;
    let stmt = conn.prepare(concat!("
        SELECT title, creation_time
        FROM poll
        WHERE poll_id = $1
        AND session_id = $2
        AND creation_time > NOW() - ", poll_duration!()
    )).await?;
    Ok(conn.query_opt(&stmt, &[poll_id, session_id]).await?.map(|row| CreatedPoll {
        title: row.get(0),
        creation_time: row.get(1),
    }))
}

pub async fn get_response_count(pool: Pool)
    -> Result<std::collections::HashMap<PollID, usize>, PoolError>
{
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT poll_id, COUNT(*)
        FROM poll_categorical_response
        GROUP BY poll_id
        UNION ALL
        SELECT poll_id, COUNT(*)
        FROM poll_numerical_response
        GROUP BY poll_id
    ").await?;
    Ok(conn.query(&stmt, &[])
        .await?
        .iter()
        .map(|row| (row.get(0), row.get::<_, i64>(1) as usize))
        .collect()
    )
}

pub async fn close_poll(pool: Pool, poll_id: &PollID) -> Result<(), PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        UPDATE poll
        SET closed = TRUE
        WHERE poll_id = $1
    ").await?;
    conn.execute(&stmt, &[poll_id]).await?;
    Ok(())
}

pub async fn poll_is_closed(pool: Pool, poll_id: &PollID) -> Result<bool, PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT closed
        FROM poll
        WHERE poll_id = $1
    ").await?;
    Ok(conn.query_one(&stmt, &[poll_id]).await?.get(0))
}
