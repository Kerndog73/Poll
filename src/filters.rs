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

pub fn configure_categorical() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("configure" / "categorical")
        .and(warp::get())
        .and(warp::fs::file("./client/dist/config_cat.html"))
}

pub fn configure_numerical() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("configure" / "numerical")
        .and(warp::get())
        .and(warp::fs::file("./client/dist/config_num.html"))
}

pub fn run() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("run" / PollID)
        .and(warp::get())
        .and(warp::fs::file("./client/dist/run.html"))
        .map(|_,f|f)
}

pub fn results() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("results" / PollID)
        .and(warp::get())
        .and(warp::fs::file("./client/dist/results.html"))
        .map(|_,f|f)
}

pub fn respond(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("respond" / PollID)
        .and(warp::get())
        .and(with_state(pool))
        .and_then(handlers::respond)
}

/*
pub fn api_configure_categorical() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "configure" / "categorical")
        .and(warp::post())
}
*/

pub fn api_configure_numerical(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "configure" / "numerical")
        .and(warp::post())
        .and(warp::body::form())
        .and(with_state(pool))
        .and_then(handlers::config_num)
}

pub fn api_respond_numerical(pool: Pool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "respond" / "numerical" / PollID)
        .and(warp::post())
        .and(warp::body::form())
        .and(with_state(pool))
        .and_then(handlers::api_respond_num)
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
    Err(rejection)
}
