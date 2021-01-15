#[macro_use]
mod utils;
mod filters;
mod handlers;
mod database;

use warp::Filter;
use deadpool_postgres::{Pool, Manager};
use deadpool_postgres::tokio_postgres::{Config, NoTls};

fn create_pool() -> Pool {
    let mut config = Config::new();
    config.host("localhost");
    config.user("postgres");
    config.dbname("poll");

    let manager = Manager::new(config, NoTls);
    Pool::new(manager, 16)
}

async fn init_database(pool: Pool) {
    let conn = pool.get().await.unwrap();
    let init = std::fs::read_to_string("initialize.sql").unwrap();
    conn.batch_execute(init.as_str()).await.unwrap();
}

#[tokio::main]
async fn main() {
    let pool = create_pool();
    init_database(pool.clone()).await;

    pretty_env_logger::init();

    let routes = filters::root()
        .or(filters::configure_categorical())
        .or(filters::configure_numerical())
        .or(filters::run())
        .or(filters::results())
        .or(filters::respond(pool.clone()))
        //.or(filters::api_configure_categorical)
        .or(filters::api_configure_numerical(pool))
        .or(filters::favicon())
        .or(filters::js())
        .or(filters::css())
        .recover(filters::leaked_rejection);

    warp::serve(routes.with(warp::log("poll")))
        .run(([0, 0, 0, 0], 80))
        .await;
}
