use askama::Template;
use crate::database as db;
use deadpool_postgres::Pool;

pub async fn respond(id: String, pool: Pool) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    if id.len() == db::POLL_ID_LENGTH {
        match id.chars().next().unwrap() {
            'c' => return respond_categorical(id, pool).await,
            'n' => return respond_numerical(id, pool).await,
            _ => {}
        }
    }
    Ok(Box::new(warp::http::StatusCode::BAD_REQUEST))
}

async fn respond_categorical(_id: String, _pool: Pool) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
}

#[derive(Template)]
#[template(path = "respond_num.html")]
struct TemplateNum {
    poll_id: db::PollID,
    title: String,
    minimum: f64,
    maximum: f64,
    integer: bool,
}

async fn respond_numerical(poll_id: db::PollID, pool: Pool) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let poll = match try_500!(db::get_poll_num(pool, &poll_id).await) {
        Some(poll) => poll,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND)),
    };

    Ok(Box::new(TemplateNum {
        poll_id,
        title: poll.title,
        minimum: poll.minimum,
        maximum: poll.maximum,
        integer: poll.integer,
    }))
}
