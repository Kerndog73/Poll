use crate::db;
use askama::Template;
use deadpool_postgres::Pool;

struct AggOption {
    name: String,
    count: usize,
    percent: f64,
}

#[derive(Template)]
#[template(path = "results_cat.html")]
struct TemplateCat {
    poll_id: db::PollID,
    title: String,
    total: usize,
    options: Vec<AggOption>,
}

pub async fn get_results_cat(poll_id: db::PollID, session_id: db::SessionID, pool: Pool)
    -> Result<Box<dyn warp::Reply>, warp::Rejection>
{
    let poll = match try_500!(db::get_poll_cat(pool.clone(), &poll_id).await) {
        Some(poll) => poll,
        None => return Ok(Box::new(warp::http::StatusCode::NOT_FOUND)),
    };
    if poll.owner != session_id {
        return Ok(Box::new(warp::http::StatusCode::NOT_FOUND));
    }

    let results = try_500!(db::get_aggregate_results_cat(pool, &poll_id).await);
    let total = results.total as f64;
    let mut options = poll.options.iter().enumerate()
        .map(|(i, name)| AggOption {
            name: name.clone(),
            count: results.histogram[i],
            percent: ((results.histogram[i] * 1000) as f64 / total).round() / 10.0,
        })
        .collect::<Vec<_>>();
    options.sort_by_key(|option| std::cmp::Reverse(option.count));

    Ok(Box::new(TemplateCat {
        poll_id,
        title: poll.title,
        total: results.total,
        options
    }))
}