use deadpool_postgres::{Pool, PoolError};
use super::{SessionID, PollID, POLL_ID_LENGTH, TITLE_LENGTH};

pub struct PollNum {
    pub owner: SessionID,
    pub title: String,
    pub minimum: f64,
    pub maximum: f64,
    pub integer: bool,
}

fn is_integer(n: f64) -> bool {
    n == n.trunc()
}

pub fn valid_poll_num(poll: &PollNum) -> bool {
    if poll.title.len() == 0 || poll.title.len() > TITLE_LENGTH { return false; }
    if poll.minimum >= poll.maximum { return false; }
    if poll.integer {
        if poll.minimum != -f64::INFINITY && !is_integer(poll.minimum) { return false; }
        if poll.maximum != f64::INFINITY && !is_integer(poll.maximum) { return false; }
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
        INSERT INTO poll_numerical (poll_id, minimum, maximum, only_integers)
        VALUES ($1, $2, $3, $4)
    ").await?;

    let poll_id = super::create_poll(&conn, super::Poll {
        owner: poll.owner,
        title: poll.title
    }).await?;

    conn.execute(&stmt, &[&poll_id, &poll.minimum, &poll.maximum, &poll.integer]).await?;

    Ok(poll_id)
}

pub async fn get_poll_num(pool: Pool, poll_id: &PollID) -> Result<Option<PollNum>, PoolError> {
    if poll_id.len() != POLL_ID_LENGTH {
        return Ok(None);
    }

    let conn = pool.get().await?;
    let stmt = conn.prepare(concat!("
        SELECT session_id, title, minimum, maximum, only_integers
        FROM poll
        JOIN poll_numerical ON poll.poll_id = poll_numerical.poll_id
        WHERE poll.poll_id = $1
        AND creation_time > NOW() - ", poll_duration!()
    )).await?;

    Ok(conn.query_opt(&stmt, &[poll_id]).await?.map(|row| PollNum {
        owner: row.get(0),
        title: row.get(1),
        minimum: row.get(2),
        maximum: row.get(3),
        integer: row.get(4),
    }))
}

pub async fn respond_poll_num(pool: Pool, poll_id: &PollID, session_id: &SessionID, res: ResponseNum) -> Result<bool, PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        INSERT INTO poll_numerical_response (poll_id, session_id, value)
        VALUES ($1, $2, $3)
        ON CONFLICT DO NOTHING
    ").await?;
    Ok(conn.execute(&stmt, &[poll_id, session_id, &res.0]).await? > 0)
}

pub async fn get_poll_results_num(pool: Pool, poll_id: &PollID) -> Result<Vec<f64>, PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT value
        FROM poll_numerical_response
        WHERE poll_id = $1
        ORDER BY value
    ").await?;
    Ok(conn.query(&stmt, &[poll_id])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect()
    )
}
