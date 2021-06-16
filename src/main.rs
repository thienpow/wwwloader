

use std::net::{SocketAddr};

mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  
    let config = config::get_configuration();
    //let server_details = "127.0.0.1:80";
    let server: SocketAddr = config.listen_on.as_str().parse().expect("Unable to parse socket address");

    //let listen_on[]: SocketAddr = "".parse();
    println!("{} is running on {}", config.service_name, config.listen_on);
    warp::serve(warp::fs::dir("www"))
        .run(server)
        .await;

    Ok(())
}
    