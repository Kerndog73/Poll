use crate::db;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use futures::{Stream, StreamExt};
use deadpool_postgres::{Pool, PoolError};
use std::collections::hash_map::{HashMap, Entry};

type Message = usize;

#[derive(Clone)]
pub struct EventContext {
    conns: Arc<RwLock<HashMap<db::PollID, Vec<mpsc::UnboundedSender<Message>>>>>,
    count: Arc<RwLock<HashMap<db::PollID, usize>>>,
}

impl EventContext {
    pub async fn new(pool: Pool) -> Result<Self, PoolError> {
        Ok(Self {
            conns: Default::default(),
            count: Arc::new(RwLock::new(db::get_response_count(pool).await?)),
        })
    }

    pub async fn add_response(&mut self, poll_id: db::PollID) {
        let mut conns_guard = self.conns.write().await;
        let mut count_guard = self.count.write().await;
        let count = count_guard.entry(poll_id.clone()).or_insert(0);
        *count += 1;

        match conns_guard.entry(poll_id) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().retain(|sender| {
                    sender.send(*count).is_ok()
                });
                if entry.get().is_empty() {
                    entry.remove();
                }
            },
            Entry::Vacant(_) => {}
        }
    }

    async fn connect(self, poll_id: db::PollID)
        -> Option<impl Stream<Item = Result<impl warp::sse::ServerSentEvent, warp::Error>> + Send + 'static>
    {
        let (ch_tx, ch_rx) = mpsc::unbounded_channel::<Message>();

        let count_guard = self.count.read().await;
        ch_tx.send(*count_guard.get(&poll_id).unwrap_or(&0)).unwrap();

        self.conns.write().await.entry(poll_id).or_default().push(ch_tx);

        Some(ch_rx.map(|message| {
            Ok(warp::sse::data(message.to_string()))
        }))
    }
}

pub async fn events(poll_id: db::PollID, session_id: db::SessionID, pool: Pool, ctx: EventContext)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    if !try_500!(db::valid_poll_id(pool, &poll_id, &session_id).await) {
        return Ok(Box::new(warp::http::StatusCode::NOT_FOUND));
    }

    match ctx.connect(poll_id).await {
        Some(stream) => Ok(Box::new(warp::sse::reply(warp::sse::keep_alive().stream(stream)))),
        None => Ok(Box::new(warp::http::StatusCode::FORBIDDEN))
    }
}
