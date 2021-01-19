use crate::db;
use askama::Template;
use deadpool_postgres::Pool;

#[derive(Template)]
#[template(path = "run.html")]
struct RunTemplate {
    title: String,
    poll_id: String,
    kind: char,
}

pub async fn get_run(kind: char, poll_id: db::PollID, session_id: db::SessionID, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    if kind != 'c' && kind != 'n' {
        return Ok(Box::new(warp::http::StatusCode::NOT_FOUND));
    }

    let title = match try_500!(db::get_poll_title(pool, &poll_id, &session_id).await) {
        Some(title) => title,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND)),
    };

    Ok(Box::new(RunTemplate {
        title,
        poll_id,
        kind,
    }))
}
