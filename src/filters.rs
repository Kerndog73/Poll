use log::debug;
use warp::Filter;
use deadpool_postgres::Pool;
use std::convert::Infallible;
use crate::{db::{PollID, SessionID}, handlers, utils};

fn with_state<S: Clone + Send>(state: S) -> impl Filter<Extract = (S,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

fn with_session_id() -> impl Filter<Extract = (SessionID,), Error = Infallible> + Clone {
    warp::any()
        .and(warp::cookie::optional("session_id"))
        .map(|session_id: Option<SessionID>| session_id.unwrap_or_default())
}

pub fn root() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and(warp::fs::file("./client/dist/home.html"))
        .map(utils::cache_long)
}

pub fn get_configure_cat() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("configure" / "c")
        .and(warp::get())
        .and(warp::fs::file("./client/dist/config_cat.html"))
        .map(utils::cache_long)
}

pub fn get_configure_num() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("configure" / "n")
        .and(warp::get())
        .and(warp::fs::file("./client/dist/config_num.html"))
        .map(utils::cache_long)
}

pub fn get_run(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("run" / char / PollID)
        .and(warp::get())
        .and(with_session_id())
        .and(with_state(pool))
        .and_then(handlers::get_run)
        .map(utils::cache_short)
}

pub fn results_cat(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("results" / "c" / PollID)
        .and(warp::get())
        .and(with_session_id())
        .and(with_state(pool))
        .and_then(handlers::get_results_cat)
        .map(utils::cache_short)
}

pub fn results_num(_pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("results_cat" / "n" / PollID)
        .and(warp::get())
        .and(warp::fs::file("./client/dist/results_cat.html"))
        .map(|_,f|f)
        .map(utils::cache_short)
}

pub fn get_respond_cat(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("respond" / "c" / PollID)
        .and(warp::get())
        .and(with_state(pool))
        .and_then(handlers::get_respond_cat)
        .map(utils::cache_short)
}

pub fn get_respond_num(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("respond" / "n" / PollID)
        .and(warp::get())
        .and(with_state(pool))
        .and_then(handlers::get_respond_num)
        .map(utils::cache_short)
}

pub fn post_configure_cat(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("configure" / "c")
        .and(warp::post())
        .and(with_session_id())
        .and(warp::body::form())
        .and(with_state(pool))
        .and_then(handlers::post_configure_cat)
}

pub fn post_configure_num(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("configure" / "n")
        .and(warp::post())
        .and(with_session_id())
        .and(warp::body::form())
        .and(with_state(pool))
        .and_then(handlers::post_configure_num)
}

pub fn post_respond_cat(pool: Pool, ctx: handlers::EventContext) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("respond" / "c" / PollID)
        .and(warp::post())
        .and(with_session_id())
        .and(warp::body::form())
        .and(with_state(pool))
        .and(with_state(ctx))
        .and_then(handlers::post_respond_cat)
}

pub fn post_respond_num(pool: Pool, ctx: handlers::EventContext) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("respond" / "n" / PollID)
        .and(warp::post())
        .and(with_session_id())
        .and(warp::body::form())
        .and(with_state(pool))
        .and(with_state(ctx))
        .and_then(handlers::post_respond_num)
}

pub fn get_csv_cat(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("csv" / "c" / PollID)
        .and(warp::get())
        .and(with_session_id())
        .and(with_state(pool))
        .and_then(handlers::get_csv_cat)
        .map(utils::cache_short)
}

pub fn get_csv_num(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("csv" / "n" / PollID)
        .and(warp::get())
        .and(with_session_id())
        .and(with_state(pool))
        .and_then(handlers::get_csv_num)
        .map(utils::cache_short)
}

pub fn events(pool: Pool, ctx: handlers::EventContext) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("events" / PollID)
        .and(warp::get())
        .and(with_session_id())
        .and(with_state(pool))
        .and(with_state(ctx))
        .and_then(handlers::events)
}

pub fn get_qr() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("qr" / char / PollID)
        .and(warp::get())
        .and_then(handlers::get_qr)
        .map(utils::cache_short)
}

pub fn favicon() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("favicon.ico")
        .and(warp::get())
        .and(warp::fs::file("./client/public/favicon.ico"))
        .map(utils::cache_long)
}

pub fn js() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("js")
        .and(warp::get())
        .and(warp::fs::dir("./client/dist/js"))
        .map(utils::cache_long)
}

pub fn css() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("css")
        .and(warp::get())
        .and(warp::fs::dir("./client/dist/css"))
        .map(utils::cache_long)
}

pub async fn leaked_rejection(rejection: warp::Rejection) -> Result<warp::http::StatusCode, warp::Rejection> {
    if rejection.is_not_found() {
        Ok(warp::http::StatusCode::NOT_FOUND)
    } else {
        debug!("{:?}", rejection);
        Ok(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}
