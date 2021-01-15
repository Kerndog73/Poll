use serde::Deserialize;
use crate::database as db;
use deadpool_postgres::Pool;

#[derive(Deserialize)]
pub struct RespondNumRequest {
    response: f64,
}

pub async fn api_respond_num(poll_id: db::PollID, req: RespondNumRequest, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    let poll = match try_500!(db::get_poll_num(pool.clone(), &poll_id).await) {
        Some(poll) => poll,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };

    let response = db::ResponseNum(req.response);

    if !db::valid_response_num(&poll, response) {
        return Ok(Box::new(warp::http::StatusCode::BAD_REQUEST));
    }

    try_500!(db::respond_poll_num(pool, poll_id, response).await);

    Ok(Box::new(warp::http::StatusCode::NO_CONTENT))
}
