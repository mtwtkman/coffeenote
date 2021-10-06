use std::sync::Arc;
use uuid::Uuid;
use crate::usecases::Usecase;
use crate::repositories::region::{DeleteError, RegionRepository};
use crate::entities::region::RegionId;

#[derive(Debug)]
pub enum Error {
    RepositoryError(DeleteError),
}

pub struct Request {
    id: Uuid,
}

impl Request {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

pub enum Response {
    Ok,
}

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
        let region_id = RegionId(req.id);
        self.repo.delete(region_id).map(|_| Response::Ok).map_err(Error::RepositoryError)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use uuid::Uuid;
    use crate::tests::harness::in_memory_repositories::region::InMemory;
    use crate::entities::region::Region;
    use crate::usecases::Usecase;
    use crate::repositories::region::DeleteError;
    use super::{DeleteRegion, Request, Error};

    #[test]
    fn it_should_delete() {
        let regions = ["xxx", "yyy"].iter().map(|name| Region::new(*name)).collect::<Vec<Region>>();
        let repo = Arc::new(InMemory::new(regions.clone(), false));
        let delete_region = DeleteRegion::new(repo.clone());
        let target = regions[0].clone();
        let req = Request::new(*target.id);
        assert!(delete_region.execute(req).is_ok());
        let after_deleted = repo.regions.lock().unwrap();
        assert_eq!(after_deleted.to_owned(), vec![regions[1].clone()]);
    }

    #[test]
    fn it_should_fail_by_an_unknown_error() {
        let repo = InMemory::new(vec![], true);
        let req = Request::new(Uuid::new_v4());
        let delete_region = DeleteRegion::new(Arc::new(repo));
        match delete_region.execute(req) {
            Err(Error::RepositoryError(DeleteError::Unknown)) => true,
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_fail_by_non_existed() {
        let regions = vec![Region::new("xxx")];
        let repo = InMemory::new(regions.clone(), false);
        let req = Request::new(Uuid::new_v4());
        let delete_region = DeleteRegion::new(Arc::new(repo));
        match delete_region.execute(req) {
            Err(Error::RepositoryError(DeleteError::NotFound)) => true,
            _ => unreachable!(),
        };
    }
}