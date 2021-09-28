use super::CoffeeNoteRepository;
use crate::entities::{
    production_area::{ProductionArea, ProductionAreaId, ProductionAreaName},
    region::Region,
};

#[derive(Debug)]
pub enum FetchOneError {
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
    name: ProductionAreaName,
    region: Region,
}

impl NewProductionArea {
    pub fn new(name: ProductionAreaName, region: Region) -> Self {
        Self { name, region }
    }
}

pub trait ProductionAreaRepository: CoffeeNoteRepository<ProductionArea> {
    fn fetch_one(&self, id: ProductionAreaId) -> Result<ProductionArea, FetchOneError>;
    fn fetch_all(&self) -> Result<Vec<ProductionArea>, FetchAllError>;
    fn create(&self, value: NewProductionArea) -> Result<ProductionArea, CreateError>;
    fn delete(&self, id: ProductionAreaId) -> Result<(), DeleteError>;
}
