use uuid::Uuid;
use std::sync::Arc;
use crate::{
    repositories::region::{RegionRepository, FetchOneByRegionIdError},
    usecases::Usecase,
    entities::region::{Region, RegionId},
};

#[derive(Debug)]
pub enum Error {
    InvalidId,
    RepositoryError(FetchOneByRegionIdError),
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
        let region_id = RegionId(req.id);
        match self.repo.fetch_one_by_region_id(region_id) {
            Ok(region) => Ok(Response { region: Some(region) }),
            Err(FetchOneByRegionIdError::NotFound) => Ok(Response { region: None }),
            Err(e) => Err(Error::RepositoryError(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::{FetchOneByRegionId, Request, Response};
    use crate::{entities::region::Region, tests::harness::in_memory_repositories::region::InMemory, usecases::Usecase};

    #[test]
    fn it_should_fetch_one_region_by_its_id() {
        let region = Region::new("xxx");
        let repo = InMemory::new(vec![region.clone()], false);
        let fetch_one_by_region_id = FetchOneByRegionId::new(Arc::new(repo));
        let req = Request { id: *(region.clone()).id};
        match fetch_one_by_region_id.execute(req) {
            Ok(Response { region: fetched }) => assert_eq!(fetched, Some(region)),
            _ => unreachable!(),
        }
    }
}
