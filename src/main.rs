mod filters;
mod handlers;

use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = filters::root()
        .or(filters::configure_categorical())
        .or(filters::configure_numerical())
        .or(filters::run())
        .or(filters::results())
        .or(filters::respond())
        //.or(filters::api_configure_categorical)
        .or(filters::api_configure_numerical())
        .or(filters::favicon())
        .or(filters::js())
        .or(filters::css())
        .recover(filters::leaked_rejection);

    warp::serve(routes.with(warp::log("poll")))
        .run(([0, 0, 0, 0], 80))
        .await;
}
