use crate::entities::{
    production_area::{ProductionArea, ProductionAreaId},
    region::Region,
};

#[derive(Debug)]
pub enum FetchOneByIdError {
    Unknown,
    NotFound,
}

#[derive(Debug)]
pub enum FetchAllError {
    Unknown,
}

#[derive(Debug)]
pub enum CreateError {
    Unknown,
    DuplicatedName,
}

#[derive(Debug)]
pub enum DeleteError {
    Unknown,
    NotFound,
}

pub struct NewProductionArea {
    pub name: String,
    pub region: Region,
}

impl NewProductionArea {
    pub fn new(name: String, region: Region) -> Self {
        Self { name, region }
    }
}

pub trait ProductionAreaRepository {
    fn fetch_one_by_id(&self, id: ProductionAreaId) -> Result<ProductionArea, FetchOneByIdError>;
    fn fetch_all(&self) -> Result<Vec<ProductionArea>, FetchAllError>;
    fn create(&self, value: NewProductionArea) -> Result<ProductionArea, CreateError>;
    fn delete(&self, id: ProductionAreaId) -> Result<(), DeleteError>;
}
