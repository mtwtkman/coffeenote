use crate::entities::{
    production_area::{ProductionArea, ProductionAreaId},
    region::Region,
};
use async_trait::async_trait;

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
    pub name: String,
    pub region: Region,
}

impl NewProductionArea {
    pub fn new(name: String, region: Region) -> Self {
        Self { name, region }
    }
}

#[async_trait]
pub trait ProductionAreaRepository {
    async fn fetch_one(&self, id: ProductionAreaId) -> Result<ProductionArea, FetchOneError>;
    async fn fetch_all(&self) -> Result<Vec<ProductionArea>, FetchAllError>;
    async fn create(&self, value: NewProductionArea) -> Result<ProductionArea, CreateError>;
    async fn delete(&self, id: ProductionAreaId) -> Result<(), DeleteError>;
}
