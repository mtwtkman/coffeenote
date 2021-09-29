use async_trait::async_trait;
use crate::entities::region::{Region, RegionId, RegionName};

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

pub struct NewRegion {
    pub name: String,
}

impl NewRegion {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[async_trait]
pub trait RegionRepository {
    async fn fetch_one(&self, id: RegionId) -> Result<Region, FetchOneError>;
    async fn fetch_all(&self) -> Result<Vec<Region>, FetchAllError>;
    async fn create(&self, value: NewRegion) -> Result<Region, CreateError>;
    async fn delete(&self, id: RegionId) -> Result<(), DeleteError>;
}
