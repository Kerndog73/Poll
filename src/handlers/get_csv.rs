use crate::db;
use deadpool_postgres::Pool;

fn reply_csv(writer: csv::Writer::<Vec<u8>>) -> impl warp::Reply {
    let buffer = writer.into_inner().unwrap();
    let string;
    unsafe { string = String::from_utf8_unchecked(buffer); }
    warp::reply::with_header(
        string,
        "Content-Type",
        "text/csv;charset=utf-8"
    )
}

pub async fn get_csv_cat(poll_id: db::PollID, session_id: db::SessionID, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    let poll = match try_500!(db::get_poll_cat(pool.clone(), &poll_id).await) {
        Some(title) => title,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };
    if poll.owner != session_id {
        return Ok(Box::new(warp::http::StatusCode::NOT_FOUND));
    }

    let mut writer = csv::WriterBuilder::new().flexible(true).from_writer(vec![]);

    writer.write_record(&[poll.title]).unwrap();

    if poll.mutex {
        let results = try_500!(db::get_aggregate_results_cat(pool, &poll_id).await);
        let mut options = results.histogram.iter()
            .take(poll.options.len())
            .enumerate()
            .collect::<Vec<_>>();
        options.sort_by_key(|option| std::cmp::Reverse(option.1));

        for option in options {
            writer.serialize((&poll.options[option.0], option.1)).unwrap();
        }
        writer.serialize(("total", results.total)).unwrap();
    } else {
        let results = try_500!(db::get_poll_results_cat(pool, &poll_id).await);

        writer.write_record(&poll.options).unwrap();
        for response in results.iter() {
            for i in 0..poll.options.len() {
                if (response >> i) & 1 == 1 {
                    writer.write_field("1").unwrap();
                } else {
                    writer.write_field("").unwrap();
                }
            }
            writer.write_record(None::<&[u8]>).unwrap();
        }
    }

    Ok(Box::new(reply_csv(writer)))
}

pub async fn get_csv_num(poll_id: db::PollID, session_id: db::SessionID, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    let poll = match try_500!(db::get_poll(pool.clone(), &poll_id, &session_id).await) {
        Some(poll) => poll,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND))
    };
    let results = try_500!(db::get_poll_results_num(pool, &poll_id).await);

    let mut writer = csv::WriterBuilder::new().from_writer(vec![]);

    writer.write_record(&[poll.title]).unwrap();
    for result in results.iter() {
        writer.serialize((result,)).unwrap()
    }

    Ok(Box::new(reply_csv(writer)))
}
