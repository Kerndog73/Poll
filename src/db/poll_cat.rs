use deadpool_postgres::{Pool, PoolError};
use super::{SessionID, PollID, POLL_ID_LENGTH};

pub const OPTION_LENGTH: usize = 64;
pub const OPTION_COUNT: usize = 16;

pub struct PollCat {
    pub owner: SessionID,
    pub title: String,
    pub mutex: bool,
    pub options: Vec<String>,
}

pub struct ResponseCat(pub i32);

pub async fn create_poll_cat(pool: Pool, poll: PollCat) -> Result<PollID, PoolError> {
    let conn = pool.get().await?;
    let poll_stmt = conn.prepare("
        INSERT INTO poll_categorical (poll_id, mutex)
        VALUES ($1, $2)
        ON CONFLICT (poll_id) DO NOTHING
    ").await?;
    let option_stmt = conn.prepare("
        INSERT INTO poll_categorical_option (poll_id, sequence, name)
        VALUES ($1, $2, $3)
    ").await?;

    let poll_id = super::create_poll(&conn, super::Poll {
        owner: poll.owner,
        title: poll.title,
    }).await?;

    conn.execute(&poll_stmt, &[&poll_id, &poll.mutex]).await?;

    for i in 0..poll.options.len() {
        conn.execute(&option_stmt, &[&poll_id, &(i as i32), &poll.options[i]]).await?;
    }

    Ok(poll_id)
}

pub async fn get_poll_cat(pool: Pool, poll_id: &PollID) -> Result<Option<PollCat>, PoolError> {
    if poll_id.len() != POLL_ID_LENGTH {
        return Ok(None);
    }

    let conn = pool.get().await?;
    let poll_stmt = conn.prepare(concat!("
        SELECT session_id, title, mutex
        FROM poll
        JOIN poll_categorical ON poll.poll_id = poll_categorical.poll_id
        WHERE poll.poll_id = $1
        AND creation_time > NOW() - ", poll_duration!()
    )).await?;
    let option_stmt = conn.prepare("
        SELECT name
        FROM poll_categorical_option
        WHERE poll_id = $1
        ORDER BY sequence
    ").await?;

    let poll = conn.query_opt(&poll_stmt, &[poll_id]).await?.map(|row| PollCat {
        owner: row.get(0),
        title: row.get(1),
        mutex: row.get(2),
        options: Vec::new(),
    });
    let mut poll = match poll {
        Some(poll) => poll,
        None => return Ok(None)
    };

    poll.options = conn.query(&option_stmt, &[poll_id])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect();

    Ok(Some(poll))
}

pub async fn respond_poll_cat(pool: Pool, poll_id: &PollID, session_id: &SessionID, res: ResponseCat) -> Result<bool, PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        INSERT INTO poll_categorical_response (poll_id, session_id, value)
        VALUES ($1, $2, $3)
        ON CONFLICT DO NOTHING
    ").await?;
    Ok(conn.execute(&stmt, &[poll_id, session_id, &res.0]).await? > 0)
}

pub async fn get_poll_results_cat(pool: Pool, poll_id: &PollID) -> Result<Vec<i32>, PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT value
        FROM poll_categorical_response
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

pub struct AggResultsCat {
    pub total: usize,
    pub histogram: Vec<usize>,
}

pub async fn get_aggregate_results_cat(pool: Pool, poll_id: &PollID) -> Result<AggResultsCat, PoolError> {
    // static_assert!(OPTION_COUNT == 16)
    let _: [u8; OPTION_COUNT] = <[u8; 16] as Default>::default();

    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT
            COUNT(*),
            COUNT(*) FILTER (WHERE value & 1 = 1),
            COUNT(*) FILTER (WHERE (value >> 1) & 1 = 1),
            COUNT(*) FILTER (WHERE (value >> 2) & 1 = 1),
            COUNT(*) FILTER (WHERE (value >> 3) & 1 = 1),
            COUNT(*) FILTER (WHERE (value >> 4) & 1 = 1),
            COUNT(*) FILTER (WHERE (value >> 5) & 1 = 1),
            COUNT(*) FILTER (WHERE (value >> 6) & 1 = 1),
            COUNT(*) FILTER (WHERE (value >> 7) & 1 = 1),
            COUNT(*) FILTER (WHERE (value >> 8) & 1 = 1),
            COUNT(*) FILTER (WHERE (value >> 9) & 1 = 1),
            COUNT(*) FILTER (WHERE (value >> 10) & 1 = 1),
            COUNT(*) FILTER (WHERE (value >> 11) & 1 = 1),
            COUNT(*) FILTER (WHERE (value >> 12) & 1 = 1),
            COUNT(*) FILTER (WHERE (value >> 13) & 1 = 1),
            COUNT(*) FILTER (WHERE (value >> 14) & 1 = 1),
            COUNT(*) FILTER (WHERE value >> 15 = 1)
        FROM poll_categorical_response
        WHERE poll_id = $1
    ").await?;

    let row = conn.query_one(&stmt, &[poll_id]).await?;
    let total = row.get::<_, i64>(0) as usize;

    let mut histogram = vec![0; OPTION_COUNT];
    for i in 0..OPTION_COUNT {
        histogram[i] = row.get::<_, i64>(i + 1) as usize;
    }

    Ok(AggResultsCat { total, histogram })
}
