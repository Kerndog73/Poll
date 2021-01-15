use serde::Deserialize;
use crate::database as db;
use deadpool_postgres::Pool;

#[derive(Deserialize)]
pub struct ConfigureNumRequest {
    title: String,
    minimum: Option<f64>,
    maximum: Option<f64>,
    integer: Option<String>,
}

pub async fn post_configure_num(req: ConfigureNumRequest, pool: Pool) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let poll = db::PollNum {
        title: req.title,
        minimum: req.minimum.unwrap_or(-f64::INFINITY),
        maximum: req.maximum.unwrap_or(f64::INFINITY),
        integer: req.integer.is_some(),
    };

    if !db::valid_poll_num(&poll) {
        return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST));
    }

    let poll_id = try_500!(db::create_poll_num(pool, poll).await);
    Ok(Box::new(warp::redirect(
        warp::http::Uri::from_maybe_shared(
            format!("/run/n/{}", poll_id)
        ).unwrap()
    )))
}
