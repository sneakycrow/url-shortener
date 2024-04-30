pub mod shortener {
    tonic::include_proto!("shortener");
}

use shortener::shortener_server::{Shortener, ShortenerServer};
use shortener::{
    CreateLinkRequest, DeleteLinkRequest, ReadLinkRequest, ShortenerResponse, UpdateLinkRequest,
};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct SneakyCrowShortener {}

#[tonic::async_trait]
impl Shortener for SneakyCrowShortener {
    async fn create(
        &self,
        _req: Request<CreateLinkRequest>,
    ) -> Result<Response<ShortenerResponse>, Status> {
        todo!("implement")
    }
    async fn read(
        &self,
        _req: Request<ReadLinkRequest>,
    ) -> Result<Response<ShortenerResponse>, Status> {
        todo!("implement")
    }
    async fn update(
        &self,
        _req: Request<UpdateLinkRequest>,
    ) -> Result<Response<ShortenerResponse>, Status> {
        todo!("implement")
    }
    async fn delete(
        &self,
        _req: Request<DeleteLinkRequest>,
    ) -> Result<Response<ShortenerResponse>, Status> {
        todo!("implement")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let shortener_service = SneakyCrowShortener::default();

    println!("ShortenerServer listening on {}", addr);

    Server::builder()
        .add_service(ShortenerServer::new(shortener_service))
        .serve(addr)
        .await?;

    Ok(())
}
