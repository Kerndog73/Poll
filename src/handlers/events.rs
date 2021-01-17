use std::sync::Arc;
use crate::database as db;
use deadpool_postgres::Pool;
use tokio::sync::{RwLock, mpsc};
use futures::{Stream, StreamExt};
use std::collections::hash_map::{HashMap, Entry};

type Message = usize;

#[derive(Clone, Default)]
pub struct EventContext {
    conns_num: Arc<RwLock<HashMap<db::PollID, mpsc::UnboundedSender<Message>>>>
}

async fn user_connected(ctx: EventContext, poll_id: db::PollID)
    -> Option<impl Stream<Item = Result<impl warp::sse::ServerSentEvent, warp::Error>> + Send + 'static>
{
    let (ch_tx, ch_rx) = mpsc::unbounded_channel::<Message>();

    ch_tx.send(42).unwrap();

    match ctx.conns_num.write().await.entry(poll_id) {
        Entry::Occupied(_) => return None,
        Entry::Vacant(entry) => entry.insert(ch_tx)
    };

    Some(ch_rx.map(|message| {
        Ok(warp::sse::data(message.to_string()))
    }))
}

pub async fn events_num(poll_id: db::PollID, session_id: db::SessionID, pool: Pool, ctx: EventContext)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    if !try_500!(db::valid_poll_id_num(pool, &poll_id, &session_id).await) {
        return Ok(Box::new(warp::http::StatusCode::NOT_FOUND));
    }

    match user_connected(ctx, poll_id).await {
        Some(stream) => Ok(Box::new(warp::sse::reply(warp::sse::keep_alive().stream(stream)))),
        None => Ok(Box::new(warp::http::StatusCode::FORBIDDEN))
    }
}
