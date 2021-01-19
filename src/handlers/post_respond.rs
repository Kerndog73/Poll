use crate::utils;
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

pub async fn post_respond_num(
    poll_id: db::PollID,
    mut session_id: db::SessionID,
    req: RespondNumRequest,
    pool: Pool,
    mut ctx: super::EventContext
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let set_cookie = try_500!(utils::create_session_if_invalid(pool.clone(), &mut session_id).await);

    let poll = match try_500!(db::get_poll_num(pool.clone(), &poll_id).await) {
        Some(poll) => poll,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };

    let response = db::ResponseNum(req.response);

    if !db::valid_response_num(&poll, response) {
        return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST));
    }

    let reply = if try_500!(db::respond_poll_num(pool, &poll_id, &session_id, response).await) {
        ctx.add_response(poll_id).await;
        StatusTemplate { message: "Success!" }
    } else {
        StatusTemplate { message: "Cannot respond more than once" }
    };

    Ok(utils::maybe_set_session_cookie(reply, session_id, set_cookie))
}
