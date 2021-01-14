macro_rules! try_500 {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                return Ok(Box::new(warp::http::StatusCode::INTERNAL_SERVER_ERROR));
            }
        }
    }
}
