use uuid::Uuid;
use std::sync::Arc;
use crate::{
    repositories::region::RegionRepository,
    usecases::Usecase,
    entities::region::Region,
};

#[derive(Debug)]
pub enum Error {
    InvalidId,
}

pub struct Request {
    id: Uuid,
}
pub struct Response {
    region: Option<Region>,
}

pub struct FetchOneByRegionId {
    repo: Arc<dyn RegionRepository>,
}

impl FetchOneByRegionId {
    pub fn new(repo: Arc<dyn RegionRepository>) -> Self {
        Self { repo }
    }
}

impl Usecase for FetchOneByRegionId {
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn execute(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        todo!();
    }
}

#[cfg(test)]
mod tests {}