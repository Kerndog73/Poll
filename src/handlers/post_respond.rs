use askama::Template;
use crate::{db, utils};
use serde::Deserialize;
use deadpool_postgres::Pool;

#[derive(Template)]
#[template(path = "status.html")]
struct StatusTemplate {
    message: &'static str,
}

pub type RespondCatRequest = Vec<(String, u32)>;

fn parse_response_cat(poll: db::PollCat, req: RespondCatRequest) -> Option<db::ResponseCat> {
    if poll.mutex {
        if req.len() != 1 { return None; }
    } else {
        if req.len() > poll.options.len() { return None; }
    }
    let mut set = 0;
    for option in req.iter() {
        if option.0 != "option" { return None; }
        if option.1 >= poll.options.len() as u32 { return None; }
        set |= 1 << option.1;
    }
    Some(db::ResponseCat(set))
}

pub async fn post_respond_cat(
    poll_id: db::PollID,
    mut session_id: db::SessionID,
    req: RespondCatRequest,
    pool: Pool,
    mut ctx: super::EventContext
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let poll = match try_500!(db::get_poll_cat(pool.clone(), &poll_id).await) {
        Some(poll) => poll,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };

    let response = match parse_response_cat(poll, req) {
        Some(response) => response,
        None => return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST))
    };

    let set_cookie = try_500!(utils::create_session_if_invalid(pool.clone(), &mut session_id).await);

    let reply = if try_500!(db::poll_is_closed(pool.clone(), &poll_id).await) {
        StatusTemplate { message: "Cannot respond to closed poll" }
    } else if try_500!(db::respond_poll_cat(pool, &poll_id, &session_id, response).await) {
        ctx.add_response(poll_id).await;
        StatusTemplate { message: "Success!" }
    } else {
        StatusTemplate { message: "Cannot respond more than once" }
    };

    Ok(utils::maybe_set_session_cookie(reply, session_id, set_cookie))
}

#[derive(Deserialize)]
pub struct RespondNumRequest {
    response: f64,
}

fn parse_response_num(poll: db::PollNum, req: RespondNumRequest) -> Option<db::ResponseNum> {
    if req.response < poll.minimum { return None; }
    if req.response > poll.maximum { return None; }
    if poll.integer && !utils::is_integer(req.response) { return None; }
    Some(db::ResponseNum(req.response))
}

pub async fn post_respond_num(
    poll_id: db::PollID,
    mut session_id: db::SessionID,
    req: RespondNumRequest,
    pool: Pool,
    mut ctx: super::EventContext
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let poll = match try_500!(db::get_poll_num(pool.clone(), &poll_id).await) {
        Some(poll) => poll,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };

    let response = match parse_response_num(poll, req) {
        Some(response) => response,
        None => return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST))
    };

    let set_cookie = try_500!(utils::create_session_if_invalid(pool.clone(), &mut session_id).await);

    let reply = if try_500!(db::poll_is_closed(pool.clone(), &poll_id).await) {
        StatusTemplate { message: "Cannot respond to closed poll" }
    } else if try_500!(db::respond_poll_num(pool, &poll_id, &session_id, response).await) {
        ctx.add_response(poll_id).await;
        StatusTemplate { message: "Success!" }
    } else {
        StatusTemplate { message: "Cannot respond more than once" }
    };

    Ok(utils::maybe_set_session_cookie(reply, session_id, set_cookie))
}
