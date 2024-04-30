use dotenvy::dotenv;
use shortener::shortener_server::ShortenerServer;
use tonic::transport::Server;

pub mod shortener {
    tonic::include_proto!("shortener");
}
pub mod db;
pub mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let shortener_service = server::SneakyCrowShortener::default();

    match dotenv() {
        Err(_) => println!("Could not load .env file"),
        Ok(_) => println!("Successfully loaded .env"),
    };

    println!("ShortenerServer listening on {}", addr);

    Server::builder()
        .add_service(ShortenerServer::new(shortener_service))
        .serve(addr)
        .await?;

    Ok(())
}
