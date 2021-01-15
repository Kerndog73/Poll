use crate::utils;
use deadpool_postgres::{Pool, PoolError};

pub const POLL_ID_LENGTH: usize = 8;
pub type PollID = String;

macro_rules! poll_duration {
    () => { "INTERVAL '1 day'" }
}

pub struct PollNum {
    pub title: String,
    pub minimum: f64,
    pub maximum: f64,
    pub integer: bool,
}

fn is_integer(n: f64) -> bool {
    (n as i64) as f64 == n
}

pub fn valid_poll_num(poll: &PollNum) -> bool {
    if poll.title.len() == 0 || poll.title.len() > 128 { return false; }
    if poll.minimum >= poll.maximum { return false; }
    if poll.integer {
        if poll.minimum == -f64::INFINITY || is_integer(poll.minimum) { return false; }
        if poll.maximum == f64::INFINITY || is_integer(poll.maximum) { return false; }
    }
    true
}

#[derive(Clone, Copy)]
pub struct ResponseNum(pub f64);

pub fn valid_response_num(poll: &PollNum, response: ResponseNum) -> bool {
    if response.0 < poll.minimum { return false; }
    if response.0 > poll.maximum { return false; }
    if poll.integer && !is_integer(response.0) { return false; }
    true
}

pub async fn create_poll_num(pool: Pool, poll: PollNum) -> Result<PollID, PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        INSERT INTO poll_numerical (poll_id, creation_time, title, minimum, maximum, only_integers)
        VALUES ($1, NOW(), $2, $3, $4, $5)
        ON CONFLICT (poll_id) DO NOTHING
    ").await?;

    let mut poll_id = utils::generate_random_base64url(POLL_ID_LENGTH);
    while conn.execute(&stmt, &[&poll_id, &poll.title, &poll.minimum, &poll.maximum, &poll.integer]).await? == 0 {
        poll_id = utils::generate_random_base64url(POLL_ID_LENGTH);
    }

    Ok(poll_id)
}

pub async fn get_poll_num(pool: Pool, poll_id: &PollID) -> Result<Option<PollNum>, PoolError> {
    if poll_id.len() != POLL_ID_LENGTH {
        return Ok(None);
    }

    let conn = pool.get().await?;
    let stmt = conn.prepare(concat!("
        SELECT title, minimum, maximum, only_integers
        FROM poll_numerical
        WHERE poll_id = $1
        AND creation_time > NOW() - ", poll_duration!())
    ).await?;

    Ok(conn.query_opt(&stmt, &[&poll_id]).await?.map(|row| PollNum {
        title: row.get(0),
        minimum: row.get(1),
        maximum: row.get(2),
        integer: row.get(3),
    }))
}

pub async fn respond_poll_num(pool: Pool, poll_id: PollID, res: ResponseNum) -> Result<(), PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        INSERT INTO poll_numerical_response (poll_id, value)
        VALUES ($1, $2)
    ").await?;
    conn.execute(&stmt, &[&poll_id, &res.0]).await?;
    Ok(())
}

pub async fn get_poll_results_num(pool: Pool, poll_id: &PollID) -> Result<Vec<f64>, PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT value
        FROM poll_numerical_response
        WHERE poll_id = $1
        ORDER BY value
    ").await?;
    Ok(conn.query(&stmt, &[&poll_id])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect()
    )
}
