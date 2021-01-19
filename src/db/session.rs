use crate::utils;
use deadpool_postgres::{Pool, PoolError};

pub const SESSION_ID_LENGTH: usize = 16;
pub type SessionID = String;

pub async fn create_session(pool: Pool) -> Result<SessionID, PoolError> {
    let conn = pool.get().await?;
    let stmt = conn.prepare("
         INSERT INTO session (session_id)
         VALUES ($1)
         ON CONFLICT DO NOTHING
    ").await?;

    let mut session_id = utils::generate_random_base64url(SESSION_ID_LENGTH);
    while conn.execute(&stmt, &[&session_id]).await? == 0 {
        session_id = utils::generate_random_base64url(SESSION_ID_LENGTH);
    }

    Ok(session_id)
}

pub async fn valid_session_id(pool: Pool, session_id: &SessionID) -> Result<bool, PoolError> {
    if session_id.len() != SESSION_ID_LENGTH {
        return Ok(false);
    }

    let conn = pool.get().await?;
    let stmt = conn.prepare("
        SELECT 1
        FROM session
        WHERE session_id = $1
    ").await?;
    Ok(conn.query_opt(&stmt, &[session_id]).await?.is_some())
}
