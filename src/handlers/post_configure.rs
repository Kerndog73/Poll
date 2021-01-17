use crate::utils;
use serde::Deserialize;
use crate::database as db;
use deadpool_postgres::Pool;
//use serde::de::IntoDeserializer;

#[derive(Deserialize, Debug)]
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
    let mut set_cookie = false;
    if !try_500!(db::valid_session_id(pool.clone(), &session_id).await) {
        session_id = try_500!(db::create_session(pool.clone()).await);
        set_cookie = true;
    }

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

    if set_cookie {
        Ok(Box::new(utils::set_session_id_cookie(redirect, session_id)))
    } else {
        Ok(Box::new(redirect))
    }
}
