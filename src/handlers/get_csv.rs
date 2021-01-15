use crate::database as db;
use deadpool_postgres::Pool;

// TODO: Do we really need a library for writing CSV?
// I mean, it's CSV...

pub async fn get_csv_num(poll_id: db::PollID, pool: Pool) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let poll = match try_500!(db::get_poll_num(pool.clone(), &poll_id).await) {
        Some(poll) => poll,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };
    let results = try_500!(db::get_poll_results_num(pool, &poll_id).await);

    let mut writer = csv::WriterBuilder::new().from_writer(vec![]);

    writer.serialize((poll.title,)).unwrap();
    for result in results.iter() {
        writer.serialize((result,)).unwrap()
    }

    let buffer = writer.into_inner().unwrap();
    let string;
    unsafe { string = String::from_utf8_unchecked(buffer); }
    Ok(Box::new(warp::reply::with_header(
        string,
        "Content-Type",
        "text/csv;charset=utf-8"
    )))
}
