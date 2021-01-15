use log::debug;
use warp::Filter;
use crate::handlers;
use crate::database::PollID;
use deadpool_postgres::Pool;
use std::convert::Infallible;

fn with_state<S: Clone + Send>(state: S) -> impl Filter<Extract = (S,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

pub fn root() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and(warp::fs::file("./client/dist/home.html"))
}

pub fn get_configure_cat() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("configure" / "c")
        .and(warp::get())
        .and(warp::fs::file("./client/dist/config_cat.html"))
}

pub fn get_configure_num() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("configure" / "n")
        .and(warp::get())
        .and(warp::fs::file("./client/dist/config_num.html"))
}

pub fn run_num() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("run" / "n" / PollID)
        .and(warp::get())
        .and(warp::fs::file("./client/dist/run.html"))
        .map(|_,f|f)
}

pub fn results_num() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("results" / "n" / PollID)
        .and(warp::get())
        .and(warp::fs::file("./client/dist/results.html"))
        .map(|_,f|f)
}

pub fn get_respond_num(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("respond" / "n" / PollID)
        .and(warp::get())
        .and(with_state(pool))
        .and_then(handlers::get_respond_num)
}

pub fn post_configure_num(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("configure" / "n")
        .and(warp::post())
        .and(warp::body::form())
        .and(with_state(pool))
        .and_then(handlers::post_configure_num)
}

pub fn post_respond_num(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("respond" / "n" / PollID)
        .and(warp::post())
        .and(warp::body::form())
        .and(with_state(pool))
        .and_then(handlers::post_respond_num)
}

pub fn favicon() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("favicon.ico")
        .and(warp::get())
        .and(warp::fs::file("./client/public/favicon.ico"))
}

pub fn js() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("js")
        .and(warp::get())
        .and(warp::fs::dir("./client/dist/js"))
}

pub fn css() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("css")
        .and(warp::get())
        .and(warp::fs::dir("./client/dist/css"))
}

pub async fn leaked_rejection(rejection: warp::Rejection) -> Result<warp::http::StatusCode, warp::Rejection> {
    debug!("{:?}", rejection);
    Ok(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
}
