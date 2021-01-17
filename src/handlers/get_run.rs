use askama::Template;
use crate::database as db;
use deadpool_postgres::Pool;

#[derive(Template)]
#[template(path = "run.html")]
struct RunTemplate {
    title: String,
    poll_id: String,
    kind: char,
}

pub async fn get_run_num(poll_id: db::PollID, session_id: db::SessionID, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    let title = match try_500!(db::get_poll_title_num(pool, &poll_id, &session_id).await) {
        Some(title) => title,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND)),
    };

    Ok(Box::new(RunTemplate {
        title,
        poll_id,
        kind: 'n',
    }))
}
