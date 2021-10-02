use std::sync::Mutex;
use crate::entities::region::{Region, RegionId};
use crate::repositories::region::{
    CreateError, DeleteError, FetchAllError, FetchOneError, NewRegion, RegionRepository,
};

pub struct InMemory {
    regions: Mutex<Vec<Region>>,
    error: bool,
}

impl InMemory {
    pub fn new(regions: Vec<Region>, error: bool) -> Self {
        Self {
            regions: Mutex::new(regions),
            error,
        }
    }
}

impl RegionRepository for InMemory {
    fn fetch_one(&self, id: RegionId) -> Result<Region, FetchOneError> {
        if self.error {
            return Err(FetchOneError::Unknown);
        }
        self.regions
            .lock()
            .or(Err(FetchOneError::Unknown))
            .and_then(|locked| {
                if let Some(v) = locked.iter().find(|v| &v.id == &id) {
                    Ok(v.clone())
                } else {
                    Err(FetchOneError::NotFound)
                }
            })
    }
    fn fetch_all(&self) -> Result<Vec<Region>, FetchAllError> {
        if self.error {
            return Err(FetchAllError::Unknown);
        }
        self.regions
            .lock()
            .or(Err(FetchAllError::Unknown))
            .map(|locked| locked.clone())
    }
    fn create(&self, value: NewRegion) -> Result<Region, CreateError> {
        if self.error {
            return Err(CreateError::Unknown);
        }
        self.regions
            .lock()
            .or(Err(CreateError::Unknown))
            .and_then(|mut locked| {
                if let Some(_found) = locked.iter().find(|v| &*v.name == &value.name) {
                    return Err(CreateError::DuplicatedName);
                }
                let new_one = Region::new(value.name);
                locked.push(new_one.clone());
                Ok(new_one)
            })
    }
    fn delete(&self, id: RegionId) -> Result<(), DeleteError> {
        if self.error {
            return Err(DeleteError::Unknown);
        }
        self.regions
            .lock()
            .or(Err(DeleteError::Unknown))
            .and_then(|mut locked| {
                if let Some(index) = locked.iter().position(|v| &v.id == &id) {
                    locked.remove(index);
                    Ok(())
                } else {
                    Err(DeleteError::NotFound)
                }
            })
    }
}
