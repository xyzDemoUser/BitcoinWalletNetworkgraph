#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
mod routes;
mod handlers;
use warp::{self, Filter};

use routes::{ create_routes};
pub async fn start() {
    println!("starting http server");
/*    let warp_routes = create_routes();
    warp::serve(warp_routes)
        .run(([127, 0, 0, 1], 3000))
        .await;*/
    /// Serving the static files in the static folder.
    let route = warp::path("static").and(warp::fs::dir("static"));

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
    println!("Running on http://localhost:3000");
}
