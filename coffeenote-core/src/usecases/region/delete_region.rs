use std::sync::Arc;
use crate::usecases::Usecase;
use crate::repositories::region::{DeleteError, RegionRepository};

#[derive(Debug)]
pub enum Error {
    RepositoryError(DeleteError),
}

pub struct Request {
    id: usize,
}

impl Request {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

pub struct Response;

pub struct DeleteRegion {
    repo: Arc<dyn RegionRepository>,
}

impl Usecase for DeleteRegion {
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn execute(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
}