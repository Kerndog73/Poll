use crate::utils;
use serde::Deserialize;
use crate::database as db;
use deadpool_postgres::{Pool, PoolError};

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
    // let set_cookie = try_500!(utils::create_session_if_invalid(pool.clone(), &mut session_id).await);

    let poll = match parse_poll_cat(session_id.clone(), req) {
        Some(poll) => poll,
        None => return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST))
    };

    println!("{:?}", poll);

    Ok(Box::new(warp::http::StatusCode::NO_CONTENT))
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

pub async fn post_configure_num(mut session_id: db::SessionID, req: ConfigureNumRequest, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    let set_cookie = try_500!(utils::create_session_if_invalid(pool.clone(), &mut session_id).await);

    let minimum = match parse_or(req.minimum, -f64::INFINITY) {
        Ok(n) => n,
        Err(_) => return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST))
    };

    let maximum = match parse_or(req.maximum, f64::INFINITY) {
        Ok(n) => n,
        Err(_) => return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST))
    };

    let poll = db::PollNum {
        owner: session_id.clone(),
        title: req.title,
        minimum,
        maximum,
        integer: req.integer.is_some(),
    };

    if !db::valid_poll_num(&poll) {
        return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST));
    }

    let poll_id = try_500!(db::create_poll_num(pool, poll).await);
    let redirect = utils::redirect_string(format!("/run/n/{}", poll_id));

    Ok(utils::maybe_set_session_cookie(redirect, session_id, set_cookie))
}
