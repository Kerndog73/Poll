mod filters;

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
        .recover(filters::leaked_rejection);

    warp::serve(routes.with(warp::log("poll")))
        .run(([0, 0, 0, 0], 80))
        .await;
}
