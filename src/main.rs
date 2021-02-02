#![deny(warnings)]

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    warp::serve(warp::fs::dir("www"))
        .run(([0, 0, 0, 0], 3030))
        .await;
}