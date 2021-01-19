use crate::db;
use deadpool_postgres::{Pool, PoolError};

pub async fn create_session_if_invalid(pool: Pool, session_id: &mut db::SessionID) -> Result<bool, PoolError> {
    if !db::valid_session_id(pool.clone(), session_id).await? {
        *session_id = db::create_session(pool).await?;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn maybe_set_session_cookie<R: 'static + warp::Reply>(reply: R, session_id: db::SessionID, set: bool) -> Box<dyn warp::Reply> {
    if set {
        Box::new(super::set_session_id_cookie(reply, session_id))
    } else {
        Box::new(reply)
    }
}
