use crate::db::{establish_connection, models};
use crate::shortener::shortener_server::Shortener;
use crate::shortener::{
    CreateLinkRequest, DeleteLinkRequest, LinkResponseStatus, ReadLinkRequest, ShortenerResponse,
    UpdateLinkRequest,
};
use tonic::{Request, Response, Status};
use url::Url;
use uuid::Uuid;

#[derive(Default)]
pub struct SneakyCrowShortener {}

#[tonic::async_trait]
impl Shortener for SneakyCrowShortener {
    async fn create(
        &self,
        req: Request<CreateLinkRequest>,
    ) -> Result<Response<ShortenerResponse>, Status> {
        // Parse and validate link from request
        let target_url = match Url::parse(&req.into_inner().target_url) {
            Ok(valid_url) => valid_url,
            Err(_) => return Err(Status::invalid_argument("Invalid URL given")),
        };
        // Establish DB connections
        let mut conn = establish_connection();
        // Check if link with target url already exists
        match models::get_link_by_url(&mut conn, &target_url.to_string()) {
            // Link already exists, return it
            Some(link) => {
                let proto_link = link.into_proto();
                let response = ShortenerResponse {
                    link: Some(proto_link),
                    status: LinkResponseStatus::Found.into(),
                };

                Ok(Response::new(response))
            }
            // Link doesn't exist, create it
            None => {
                // If the link wasn't found by it's url, create it
                let db_link: models::Link = models::create_link(&mut conn, &target_url.to_string());
                let proto_link = db_link.into_proto();
                let response = ShortenerResponse {
                    link: Some(proto_link),
                    status: LinkResponseStatus::Created.into(),
                };

                Ok(Response::new(response))
            }
        }
    }
    async fn read(
        &self,
        req: Request<ReadLinkRequest>,
    ) -> Result<Response<ShortenerResponse>, Status> {
        let target_link_id = req.into_inner().id;
        let target_link_uuid = match Uuid::parse_str(&target_link_id) {
            Ok(uuid) => uuid,
            Err(_) => return Err(Status::invalid_argument("Valid UUID not provided")),
        };
        let mut conn = establish_connection();
        let found_link = models::get_link_by_id(&mut conn, target_link_uuid);
        match found_link {
            Some(link) => {
                let proto_link = link.into_proto();
                let response = ShortenerResponse {
                    link: Some(proto_link),
                    status: LinkResponseStatus::Found.into(),
                };

                Ok(Response::new(response))
            }
            None => Err(Status::not_found("Could not find link by id")),
        }
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
