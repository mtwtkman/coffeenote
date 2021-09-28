use crate::entities::{production_area::{ProductionAreaName, ProductionArea}, region::Region, Invalid, Validate};
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

    pub async fn exec(&self, req: Request) -> Result<Response, Error> {
        let name = ProductionAreaName::from(req.name.clone());
        if let Err(invalid) = name.validate() {
            return Err(Error::InvalidParameter(invalid));
        }
        let param = NewProductionArea::new(req.name, req.region);  // TODO: I must decide that req.name should be ProductionAreaName or not.
        self.repo
            .create(param)
            .await
            .map(|production_area| Response { production_area })
            .map_err(Error::CreatError)
    }
}
