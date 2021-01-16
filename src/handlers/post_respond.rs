use askama::Template;
use serde::Deserialize;
use crate::database as db;
use deadpool_postgres::Pool;

#[derive(Deserialize)]
pub struct RespondNumRequest {
    response: f64,
}

#[derive(Template)]
#[template(path = "status.html")]
struct StatusTemplate {
    message: &'static str,
}

pub async fn post_respond_num(poll_id: db::PollID, completed: Option<String>, req: RespondNumRequest, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    if completed.is_some() {
        return Ok(Box::new(StatusTemplate {
            message: "Cannot respond more than once"
        }));
    }

    let poll = match try_500!(db::get_poll_num(pool.clone(), &poll_id).await) {
        Some(poll) => poll,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };

    let response = db::ResponseNum(req.response);

    if !db::valid_response_num(&poll, response) {
        return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST));
    }

    try_500!(db::respond_poll_num(pool, &poll_id, response).await);

    Ok(Box::new(warp::reply::with_header(
        StatusTemplate {
            message: "Success!"
        },
        "Set-Cookie",
        // Max-Age is set to 24 hours which is the lifetime of a poll
        format!("completed=;HttpOnly;Max-Age=86400;Path=/respond/n/{}", poll_id)
    )))
}
