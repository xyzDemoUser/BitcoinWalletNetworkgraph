use std::convert::Infallible;
use std::error::Error;
use warp::{self, Filter};

/// All http routes
pub fn create_routes() ->
                       impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        static_content()
        //.or(list_transactions_Wallet())
}


fn static_content() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path("static").and(warp::fs::dir("static"))
}
/*f
/// GET /api/transactions/{address}
/// A trait object.
n list_transactions_Wallet() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection>
+ Clone
{
    warp::path!("/api/transactions" / String)
        .and(warp::get())
        .and(with_value())
}

fn with_value() -> impl Filter<Extract =() Error = warp::Rejection> + Clone {
    warp::any().map(move || None)
}*/