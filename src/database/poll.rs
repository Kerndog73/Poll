use crate::utils;
use super::SessionID;
use deadpool_postgres::{Client, Pool, PoolError};

pub const POLL_ID_LENGTH: usize = 8;
pub type PollID = String;

pub const TITLE_LENGTH: usize = 128;

macro_rules! poll_duration {
    () => { "INTERVAL '10 day'" }
}

pub struct Poll {
    pub owner: SessionID,
    pub title: String,
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

pub async fn get_poll_title(pool: Pool, poll_id: &PollID, session_id: &SessionID)
    -> Result<Option<String>, PoolError>
{
    let conn = pool.get().await?;
    let stmt = conn.prepare(concat!("
        SELECT title
        FROM poll
        WHERE poll_id = $1
        AND session_id = $2
        AND creation_time > NOW() - ", poll_duration!()
    )).await?;
    Ok(conn.query_opt(&stmt, &[poll_id, session_id]).await?.map(|row| row.get(0)))
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
