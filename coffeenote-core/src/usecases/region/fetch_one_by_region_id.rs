use uuid::Uuid;
use std::sync::Arc;
use crate::{
    repositories::region::{RegionRepository, FetchOneByRegionIdError},
    usecases::Usecase,
    entities::region::{Region, RegionId},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidId,
    RepositoryError(FetchOneByRegionIdError),
}

pub struct Request {
    id: Uuid,
}

#[derive(PartialEq, Eq, Debug)]
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
    use uuid::Uuid;
    use std::sync::Arc;
    use super::{FetchOneByRegionId, Request, Response, Error};
    use crate::{
        entities::region::Region,
        tests::harness::in_memory_repositories::region::InMemory,
        usecases::Usecase,
        repositories::region::FetchOneByRegionIdError,
    };

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

    #[test]
    fn it_should_none_by_not_found() {
        let region = Region::new("xxx");
        let repo = InMemory::new(vec![region.clone()], false);
        let fetch_one_by_region_id = FetchOneByRegionId::new(Arc::new(repo));
        let req = Request { id: Uuid::new_v4()};
        match fetch_one_by_region_id.execute(req) {
            Ok(Response { region: None }) => (),
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_happen_an_error_by_repository_issue() {
        let region = Region::new("xxx");
        let repo = InMemory::new(vec![region.clone()], true);
        let fetch_one_by_region_id = FetchOneByRegionId::new(Arc::new(repo));
        let req = Request { id: *region.id};
        assert_eq!(fetch_one_by_region_id.execute(req), Err(Error::RepositoryError(FetchOneByRegionIdError::Unknown)));
    }
}
