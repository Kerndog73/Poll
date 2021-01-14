use log::debug;
use warp::Filter;
use std::convert::Infallible;

fn with_state<S: Clone + Send>(state: S) -> impl Filter<Extract = (S,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

pub fn root() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .map(|| "Root")
}

pub fn configure_categorical() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("configure" / "categorical")
        .and(warp::get())
        .map(|| "Configure categorical")
}

pub fn configure_numerical() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("configure" / "numerical")
        .and(warp::get())
        .map(|| "Configure numerical")
}

pub fn run() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("run" / String)
        .and(warp::get())
        .map(|id| format!("Run {}", id))
}

pub fn results() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("results" / String)
        .and(warp::get())
        .map(|id| format!("Results {}", id))
}

pub fn respond() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("respond" / String)
        .and(warp::get())
        .map(|id| format!("Respond {}", id))
}

pub async fn leaked_rejection(rejection: warp::Rejection) -> Result<warp::http::StatusCode, warp::Rejection> {
    debug!("{:?}", rejection);
    Err(rejection)
}
