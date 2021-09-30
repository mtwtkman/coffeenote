use crate::entities::{
    region::{Region, RegionName},
    Invalid, Validate,
};
use crate::repositories::region::{CreateError, NewRegion, RegionRepository};
use std::sync::Arc;

#[derive(Debug)]
pub enum Error {
    CreateError(CreateError),
    InvalidParameter(Invalid<<RegionName as Validate>::ValueType>),
}

pub struct Request {
    pub name: String,
}

impl Request {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }
}

pub struct Response {
    pub region: Region,
}

pub struct CreateRegion {
    pub repo: Arc<dyn RegionRepository>,
}

impl CreateRegion {
    pub fn new(repo: Arc<dyn RegionRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, req: Request) -> Result<Response, Error> {
        let name = RegionName::from(req.name.clone());
        if let Err(invalid) = name.validate() {
            return Err(Error::InvalidParameter(invalid));
        }
        let param = NewRegion::new(req.name);
        self.repo
            .create(param)
            .await
            .map(|region| Response { region })
            .map_err(Error::CreateError)
    }
}

#[cfg(test)]
mod tests {
    use super::{CreateRegion, Request, Response, Error, CreateError};
    use crate::{
        tests::harness::in_memory_repositories::region::InMemory,
        entities::region::Region,
    };
    use std::sync::Arc;

    #[tokio::test]
    async fn it_should_create_new_one() {
        let repo = InMemory::new(vec![], false);
        let usecase = CreateRegion::new(Arc::new(repo));
        let name = "xxx";
        let req = Request::new(name);
        match usecase.execute(req).await {
            Ok(Response { region }) => {
                assert_eq!(&*region.name, name);
            }
            Err(_) => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_fail_by_duplicated_name() {
        let name = "xxx";
        let one = Region::new(name);
        let repo = InMemory::new(vec![one.clone()], false);
        let usecase = CreateRegion::new(Arc::new(repo));
        let req = Request::new(name);
        match usecase.execute(req).await {
            Err(Error::CreateError(CreateError::DuplicatedName)) => true,
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_fail_by_an_unknown_error() {
        let repo = InMemory::new(vec![], true);
        let usecase = CreateRegion::new(Arc::new(repo));
        let req = Request::new("x");
        match usecase.execute(req).await {
            Err(Error::CreateError(CreateError::Unknown)) => true,
            _ => unreachable!(),
        };
    }
}
