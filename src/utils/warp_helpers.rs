use crate::database as db;

/*
pub fn redirect_str(url: &'static str) -> impl warp::Reply {
    warp::redirect(warp::http::Uri::from_static(url))
}
*/

pub fn redirect_string(url: String) -> impl warp::Reply {
    warp::redirect(warp::http::Uri::from_maybe_shared(url).unwrap())
}

pub fn set_session_id_cookie<R: warp::Reply>(reply: R, session_id: db::SessionID) -> impl warp::Reply {
    warp::reply::with_header(
        reply,
        "Set-Cookie",
        format!("session_id={};HttpOnly;Path=/", session_id)
    )
}
