use super::CoffeeNoteRepository;

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

pub struct CreateRegion {
    name: RegionName,
}

impl CreateRegion {
    pub fn new(name: String) -> Self {
        Self {
            name: RegionName::from(name),
        }
    }
}

pub trait RegionRepository: CoffeeNoteRepository<Region> {
    fn fetch_one(&self, id: RegionId) -> Result<Region, FetchOneError>;
    fn fetch_all(&self) -> Result<Vec<Region>, FetchAllError>;
    fn create(&self, value: CreateRegion) -> Result<Region, CreateError>;
    fn delete(&self, id: RegionId) -> Result<(), DeleteError>;
}
