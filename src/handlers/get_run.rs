use crate::db;
use askama::Template;
use deadpool_postgres::Pool;

#[derive(Template)]
#[template(path = "run.html")]
struct RunTemplate {
    title: String,
    expire: u64,
    poll_id: String,
    kind: char,
}

pub async fn get_run(kind: char, poll_id: db::PollID, session_id: db::SessionID, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    if kind != 'c' && kind != 'n' {
        return Ok(Box::new(warp::http::StatusCode::NOT_FOUND));
    }

    let poll = match try_500!(db::get_poll(pool.clone(), &poll_id, &session_id).await) {
        Some(poll) => poll,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND)),
    };

    Ok(Box::new(RunTemplate {
        title: poll.title,
        expire: (poll.creation_time + db::POLL_DURATION)
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        poll_id,
        kind,
    }))
}
