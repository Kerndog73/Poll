use deadpool_postgres::{Pool, PoolError};
use super::{SessionID, PollID, POLL_ID_LENGTH};

pub struct PollNum {
    pub owner: SessionID,
    pub title: String,
    pub minimum: f64,
    pub maximum: f64,
    pub integer: bool,
}

#[derive(Clone, Copy)]
pub struct ResponseNum(pub f64);

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

pub struct AggResultsNum {
    pub minimum: f64,
    pub median: f64,
    pub mean: f64,
    pub maximum: f64,
    pub sum: f64,
    pub count: usize,
}

pub async fn get_aggregate_results_num(pool: Pool, poll_id: &PollID) -> Result<AggResultsNum, PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT
            MIN(value),
            PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY value),
            AVG(value),
            MAX(value),
            SUM(value),
            COUNT(*)
        FROM poll_numerical_response
        WHERE poll_id = $1
    ").await?;
    let row = conn.query_one(&stmt, &[poll_id]).await?;
    Ok(AggResultsNum {
        minimum: row.get(0),
        median: row.get(1),
        mean: row.get(2),
        maximum: row.get(3),
        sum: row.get(4),
        count: row.get::<_, i64>(5) as usize,
    })
}
