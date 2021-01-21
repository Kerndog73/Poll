use crate::{db, utils};
use serde::Deserialize;
use deadpool_postgres::Pool;

pub type ConfigureCatRequest = Vec<(String, String)>;

fn parse_poll_cat(session_id: db::SessionID, req: ConfigureCatRequest) -> Option<db::PollCat> {
    if req.len() < 3 || req.len() > db::OPTION_COUNT + 2 { return None; }
    if req[0].0 != "title" { return None; }
    if req[0].1.is_empty() || req[0].1.len() > db::TITLE_LENGTH { return None; }

    let mutex = if req[1].0 == "mutex" {
        if req[1].1 != "on" { return None; }
        true
    } else {
        false
    };

    if mutex && req.len() < 4 { return None; }
    if !mutex && req.len() > db::OPTION_COUNT + 1 { return None; }
    let start = 1 + mutex as usize;
    let mut options = Vec::new();

    for i in start..req.len() {
        if req[i].0 != "option" { return None; }
        if req[i].1.is_empty() || req[i].1.len() > db::OPTION_LENGTH { return None; }
        options.push(req[i].1.clone());
    }

    Some(db::PollCat {
        owner: session_id,
        title: req[0].1.clone(),
        mutex,
        options
    })
}

pub async fn post_configure_cat(mut session_id: db::SessionID, req: ConfigureCatRequest, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    let set_cookie = try_500!(utils::create_session_if_invalid(pool.clone(), &mut session_id).await);

    let poll = match parse_poll_cat(session_id.clone(), req) {
        Some(poll) => poll,
        None => return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST))
    };

    let poll_id = try_500!(db::create_poll_cat(pool.clone(), poll).await);
    let redirect = utils::redirect_string(format!("/run/c/{}", poll_id));

    Ok(utils::maybe_set_session_cookie(redirect, session_id, set_cookie))
}

#[derive(Deserialize)]
pub struct ConfigureNumRequest {
    title: String,
    minimum: String,
    maximum: String,
    integer: Option<String>,
}

fn parse_or(string: String, default: f64) -> Result<f64, std::num::ParseFloatError> {
    if string.is_empty() {
        Ok(default)
    } else {
        string.parse()
    }
}

fn parse_poll_num(session_id: db::SessionID, req: ConfigureNumRequest) -> Option<db::PollNum> {
    if req.title.len() == 0 || req.title.len() > db::TITLE_LENGTH { return None; }

    let minimum = match parse_or(req.minimum, -f64::INFINITY) {
        Ok(n) => n,
        Err(_) => return None
    };

    let maximum = match parse_or(req.maximum, f64::INFINITY) {
        Ok(n) => n,
        Err(_) => return None
    };

    let integer = req.integer.is_some();

    if minimum >= maximum { return None; }
    if integer {
        if minimum != -f64::INFINITY && !utils::is_integer(minimum) { return None; }
        if maximum != f64::INFINITY && !utils::is_integer(maximum) { return None; }
    }

    Some(db::PollNum {
        owner: session_id,
        title: req.title,
        minimum,
        maximum,
        integer,
    })
}

pub async fn post_configure_num(mut session_id: db::SessionID, req: ConfigureNumRequest, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    let set_cookie = try_500!(utils::create_session_if_invalid(pool.clone(), &mut session_id).await);

    let poll = match parse_poll_num(session_id.clone(), req) {
        Some(poll) => poll,
        None => return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST))
    };

    let poll_id = try_500!(db::create_poll_num(pool, poll).await);
    let redirect = utils::redirect_string(format!("/run/n/{}", poll_id));

    Ok(utils::maybe_set_session_cookie(redirect, session_id, set_cookie))
}
