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
    let ctx = handlers::EventContext::new(pool.clone()).await.unwrap();

    pretty_env_logger::init();

    let routes = filters::root()
        .or(filters::get_configure_cat())
        .or(filters::get_configure_num())
        .or(filters::get_run_num(pool.clone()))
        .or(filters::results_num())
        .or(filters::get_respond_num(pool.clone()))
        .or(filters::post_configure_num(pool.clone()))
        .or(filters::post_respond_num(pool.clone(), ctx.clone()))
        .or(filters::get_csv_num(pool.clone()))
        .or(filters::events_num(pool, ctx))
        .or(filters::get_qr())
        .or(filters::favicon())
        .or(filters::js())
        .or(filters::css())
        .recover(filters::leaked_rejection);

    warp::serve(routes.with(warp::log("poll")))
        .run(([0, 0, 0, 0], 80))
        .await;
}
