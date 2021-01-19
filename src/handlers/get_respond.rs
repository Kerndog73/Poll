use crate::db;
use askama::Template;
use deadpool_postgres::Pool;

#[derive(Template)]
#[template(path = "respond_cat.html")]
struct TemplateCat {
    poll_id: db::PollID,
    title: String,
    mutex: bool,
    options: Vec<String>,
}

pub async fn get_respond_cat(poll_id: db::PollID, pool: Pool) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let poll = match try_500!(db::get_poll_cat(pool, &poll_id).await) {
        Some(poll) => poll,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND)),
    };

    Ok(Box::new(TemplateCat {
        poll_id,
        title: poll.title,
        mutex: poll.mutex,
        options: poll.options,
    }))
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

pub async fn get_respond_num(poll_id: db::PollID, pool: Pool) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
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
