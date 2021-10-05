use crate::entities::{
    region::{Region, RegionName},
    Invalid, Validate,
};
use crate::repositories::region::{CreateError, NewRegion, RegionRepository};
use std::sync::Arc;
use crate::usecases::Usecase;

#[derive(Debug)]
pub enum Error {
    RepositoryError(CreateError),
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
}

impl Usecase for CreateRegion {
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn execute(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        let name = RegionName::from(req.name.clone());
        if let Err(invalid) = name.validate() {
            return Err(Error::InvalidParameter(invalid));
        }
        let param = NewRegion::new(req.name);
        self.repo
            .create(param)
            .map(|region| Response { region })
            .map_err(Error::RepositoryError)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CreateRegion,
        Request,
        Response,
        Error,
        Usecase,
        CreateError,
    };
    use crate::{
        tests::harness::in_memory_repositories::region::InMemory,
        entities::region::Region,
    };
    use std::sync::Arc;

    #[test]
    fn it_should_create_new_one() {
        let repo = InMemory::new(vec![], false);
        let usecase = CreateRegion::new(Arc::new(repo));
        let name = "xxx";
        let req = Request::new(name);
        match usecase.execute(req) {
            Ok(Response { region }) => {
                assert_eq!(&*region.name, name);
            }
            Err(_) => unreachable!(),
        };
    }

    #[test]
    fn it_should_fail_by_duplicated_name() {
        let name = "xxx";
        let one = Region::new(name);
        let repo = InMemory::new(vec![one.clone()], false);
        let usecase = CreateRegion::new(Arc::new(repo));
        let req = Request::new(name);
        match usecase.execute(req) {
            Err(Error::RepositoryError(CreateError::DuplicatedName)) => true,
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_fail_by_an_unknown_error() {
        let repo = InMemory::new(vec![], true);
        let usecase = CreateRegion::new(Arc::new(repo));
        let req = Request::new("x");
        match usecase.execute(req) {
            Err(Error::RepositoryError(CreateError::Unknown)) => true,
            _ => unreachable!(),
        };
    }
}
