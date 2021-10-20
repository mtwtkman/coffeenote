use crate::entities::region::{Region, RegionId, RegionName};

#[derive(Debug)]
pub enum FetchOneByRegionIdError {
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

#[derive(Debug)]
pub enum UpdateError {
    Unknown,
    NotFound,
    DuplicatedName,
}

pub struct NewRegion {
    pub name: String,
}

impl NewRegion {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

pub trait RegionRepository {
    fn fetch_one_by_region_id(&self, id: RegionId) -> Result<Region, FetchOneByRegionIdError>;
    fn fetch_all(&self) -> Result<Vec<Region>, FetchAllError>;
    fn create(&self, value: NewRegion) -> Result<Region, CreateError>;
    fn delete(&self, id: RegionId) -> Result<(), DeleteError>;
    fn update(&self, id: RegionId, name: RegionName) -> Result<(), UpdateError>;
}
