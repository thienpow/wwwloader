#![deny(warnings)]

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    println!("esm-admin is running on port 3030");
    warp::serve(warp::fs::dir("www"))
        .run(([0, 0, 0, 0], 3030))
        .await;
}