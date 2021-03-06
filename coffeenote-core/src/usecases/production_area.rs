use crate::entities::{
    production_area::{ProductionArea, ProductionAreaName},
    region::Region,
    Invalid, Validate,
};
use crate::repositories::production_area::{
    CreateError, NewProductionArea, ProductionAreaRepository,
};

#[derive(Debug)]
pub enum Error {
    CreatError(CreateError),
    InvalidParameter(Invalid<<ProductionAreaName as Validate>::ValueType>),
}

pub struct Request {
    pub name: String,
    pub region: Region,
}

pub struct Response {
    pub production_area: ProductionArea,
}

pub struct CreateProductionArea<R: ProductionAreaRepository> {
    pub repo: R,
}

impl<R: ProductionAreaRepository> CreateProductionArea<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn exec(&self, req: Request) -> Result<Response, Error> {
        let name = ProductionAreaName::from(req.name.clone());
        if let Err(invalid) = name.validate() {
            return Err(Error::InvalidParameter(invalid));
        }
        let param = NewProductionArea::new(req.name, req.region);
        self.repo
            .create(param)
            .map(|production_area| Response { production_area })
            .map_err(Error::CreatError)
    }
}
