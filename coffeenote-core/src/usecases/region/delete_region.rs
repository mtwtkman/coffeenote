use std::sync::Arc;
use crate::usecases::Usecase;
use crate::repositories::region::{DeleteError, RegionRepository};

#[derive(Debug)]
pub enum Error {
    RepositoryError(DeleteError),
}

pub struct Request {
    id: String,
}

impl Request {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

pub struct Response;

pub struct DeleteRegion {
    repo: Arc<dyn RegionRepository>,
}

impl DeleteRegion {
    pub fn new(repo: Arc<dyn RegionRepository>) -> Self {
        Self { repo }
    }
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
    use std::sync::Arc;
    use crate::tests::harness::in_memory_repositories::region::InMemory;
    use crate::entities::region::Region;
    use crate::usecases::Usecase;
    use super::{DeleteRegion, Request};

    fn it_should_delete() {
        let regions = ["xxx", "yyy"].iter().map(|name| Region::new(*name)).collect::<Vec<Region>>();
        let repo = Arc::new(InMemory::new(regions.clone(), false));
        let delete_region = DeleteRegion::new(repo.clone());
        let target = regions[0].clone();
        let req = Request::new(target.id.into());
        assert!(delete_region.execute(req).is_ok());
        let deleted = repo.regions.lock().unwrap();
        assert_eq!(deleted.len(), 1);
    }

    #[test]
    fn it_should_fail_by_an_unknown_error() {
    }

    #[test]
    fn it_should_fail_by_non_existed() {
    }
}