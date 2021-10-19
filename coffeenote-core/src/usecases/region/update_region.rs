use std::sync::Arc;
use uuid::Uuid;
use crate::usecases::Usecase;
use crate::repositories::region::{UpdateError, RegionRepository};
use crate::entities::{
    region::{RegionName, RegionId},
    Invalid,
    Validate,
};

#[derive(Debug)]
pub enum Error {
    RepositoryError(UpdateError),
    InvalidName(Invalid<String>),
}

pub struct Request {
    id: Uuid,
    name: String,
}

impl Request {
    pub fn new(id: Uuid, name: String) -> Self {
        Self { id, name }
    }
}

pub enum Response {
    Ok,
}

pub struct UpdateRegion {
    repo: Arc<dyn RegionRepository>,
}

impl UpdateRegion {
    pub fn new(repo: Arc<dyn RegionRepository>) -> Self {
        Self { repo }
    }
}

impl Usecase for UpdateRegion {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    fn execute(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        let name = RegionName(req.name);
        if let Err(invalid) = name.validate() {
            return Err(Error::InvalidName(invalid));
        }
        self.repo
            .update(
                RegionId::from(req.id),
                name,
            )
            .map_err(Error::RepositoryError)
            .map(|_| Response::Ok)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::entities::region::{RegionName, Region};
    use crate::tests::harness::in_memory_repositories::region::InMemory;
    use super::{Usecase, UpdateRegion, Request, Response, UpdateError, Error};

    #[test]
    fn it_should_update() {
        let regions = ["xxx", "yyy"].iter().map(|name| Region::new(*name)).collect::<Vec<Region>>();
        let repo = Arc::new(InMemory::new(regions.clone(), false));
        let target_index = 0;
        let target = (regions.clone())[target_index].clone();
        let update_region = UpdateRegion::new(repo.clone());
        let new_name = format!("updated-{}", (*target.name).clone());
        let req = Request::new(*target.id, new_name.clone());
        match update_region.execute(req) {
            Ok(Response::Ok) => {
                let updated = repo.regions.lock().unwrap();
                let not_updated = &regions.clone()[1];
                assert_eq!(vec![Region { id: target.id.clone(), name: RegionName::from(new_name.clone()) }, not_updated.to_owned()], updated.to_owned());
            },
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_happen_error_by_non_existed_one() {
        let region = Region::new("xxx");
        let repo = InMemory::new(vec![region], false);
        let update_region = UpdateRegion::new(Arc::new(repo));
        let req = Request::new(uuid::Uuid::new_v4(), "non-exsited".to_string());
        match update_region.execute(req) {
            Err(Error::RepositoryError(UpdateError::NotFound)) => (),
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_happen_error_by_unkonwn_error() {
        let region = Region::new("xxx");
        let repo = InMemory::new(vec![region.clone()], true);
        let update_region = UpdateRegion::new(Arc::new(repo));
        let req = Request::new(*region.clone().id, "yyy".to_string());
        match update_region.execute(req) {
            Err(Error::RepositoryError(UpdateError::Unknown)) => (),
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_happen_error_by_different_id_but_duplicated_name_error() {
        let regions = ["xxx", "yyy"].iter().map(|name| Region::new(*name)).collect::<Vec<Region>>();
        let repo = InMemory::new(regions.clone(), false);
        let update_region = UpdateRegion::new(Arc::new(repo));
        let target = regions[0].clone();
        let duplicated = regions[1].clone();
        let req = Request::new(*target.id, (*duplicated.name).clone());
        match update_region.execute(req) {
            Err(Error::RepositoryError(UpdateError::DuplicatedName)) => (),
            _ => unreachable!(),
        };
    }
}