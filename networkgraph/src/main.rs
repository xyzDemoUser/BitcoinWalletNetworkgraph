use std::borrow::Borrow;
use www::{start};
use warp::{Filter, Rejection, Buf};
use std::env;


#[tokio::main]
async fn main() {
    let path = env::current_dir();
    /// Static content
    let route = warp::path("static").and(warp::fs::dir("www/static"));
    println!("Running on http://localhost:3000/static/");
    println!("Current dir{}", get_current_working_dir());

    warp::serve(static_content()).run(([0, 0, 0, 0], 3000)).await;

}

/// To lazy to write this code: https://stackoverflow.com/questions/69540812/how-to-return-current-working-directory-from-function
fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

fn static_content() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path("static").and(warp::fs::dir("www/static"))
}
