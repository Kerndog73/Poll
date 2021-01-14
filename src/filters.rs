use log::debug;
use warp::Filter;
use std::convert::Infallible;

fn with_state<S: Clone + Send>(state: S) -> impl Filter<Extract = (S,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

pub fn root() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and(warp::fs::file("./client/public/home.html"))
}

pub fn configure_categorical() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("configure" / "categorical")
        .and(warp::get())
        .and(warp::fs::file("./client/public/config_cat.html"))
}

pub fn configure_numerical() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("configure" / "numerical")
        .and(warp::get())
        .and(warp::fs::file("./client/public/config_num.html"))
}

pub fn run() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("run" / String)
        .and(warp::get())
        .and(warp::fs::file("./client/public/run.html"))
        .map(|s,f|f)
}

pub fn results() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("results" / String)
        .and(warp::get())
        .and(warp::fs::file("./client/public/results.html"))
        .map(|s,f|f)
}

pub fn respond() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("respond" / String)
        .and(warp::get())
        .and(warp::fs::file("./client/public/respond_cat.html"))
        .map(|s,f|f)
}

pub fn favicon() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("favicon.ico")
        .and(warp::get())
        .and(warp::fs::file("./client/public/favicon.ico"))
}

pub async fn leaked_rejection(rejection: warp::Rejection) -> Result<warp::http::StatusCode, warp::Rejection> {
    debug!("{:?}", rejection);
    Err(rejection)
}
