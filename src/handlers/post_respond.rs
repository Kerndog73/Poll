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

pub async fn post_respond_num(poll_id: db::PollID, mut session_id: db::SessionID, req: RespondNumRequest, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    let mut set_cookie = false;
    if !try_500!(db::valid_session_id(pool.clone(), &session_id).await) {
        session_id = try_500!(db::create_session(pool.clone()).await);
        set_cookie = true;
    }

    let poll = match try_500!(db::get_poll_num(pool.clone(), &poll_id).await) {
        Some(poll) => poll,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };

    let response = db::ResponseNum(req.response);

    if !db::valid_response_num(&poll, response) {
        return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST));
    }

    let reply = if !try_500!(db::respond_poll_num(pool, &poll_id, &session_id, response).await) {
        StatusTemplate { message: "Cannot respond more than once" }
    } else {
        StatusTemplate { message: "Success!" }
    };

    if set_cookie {
        Ok(Box::new(utils::set_session_id_cookie(reply, session_id)))
    } else {
        Ok(Box::new(reply))
    }
}
