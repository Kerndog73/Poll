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

pub async fn get_run_num(poll_id: db::PollID, pool: Pool) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let poll = match try_500!(db::get_poll_num(pool, &poll_id).await) {
        Some(poll) => poll,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND)),
    };

    Ok(Box::new(RunTemplate {
        title: poll.title,
        poll_id,
        kind: 'n',
    }))
}
