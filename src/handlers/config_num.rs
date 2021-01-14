use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConfigNumRequest {
    title: Option<String>,
    minimum: Option<f64>,
    maximum: Option<f64>,
    integer: Option<String>,
}

pub async fn config_num(req: ConfigNumRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let title = req.title.unwrap_or_default();
    let minimum = req.minimum.unwrap_or(-f64::INFINITY);
    let maximum = req.maximum.unwrap_or(f64::INFINITY);
    let integer = req.integer.is_some();
    println!("Title: {}", title);
    println!("Range: {} to {}", minimum, maximum);
    println!("Integers only? {}", integer);
    Ok(warp::redirect(warp::http::Uri::from_static("/run/id")))
}
