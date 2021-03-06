use crate::entities::region::{Region, RegionId};
use crate::repositories::region::{
    CreateError, DeleteError, FetchAllError, FetchOneByRegionIdError, NewRegion, RegionRepository,
    UpdateError,
};
use std::sync::Mutex;

pub struct InMemory {
    pub regions: Mutex<Vec<Region>>,
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
    fn fetch_one_by_region_id(&self, id: RegionId) -> Result<Region, FetchOneByRegionIdError> {
        if self.error {
            return Err(FetchOneByRegionIdError::Unknown);
        }
        self.regions
            .lock()
            .or(Err(FetchOneByRegionIdError::Unknown))
            .and_then(|locked| {
                if let Some(v) = locked.iter().find(|v| &v.id == &id) {
                    Ok(v.clone())
                } else {
                    Err(FetchOneByRegionIdError::NotFound)
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

    fn update(
        &self,
        id: RegionId,
        name: crate::entities::region::RegionName,
    ) -> Result<(), UpdateError> {
        if self.error {
            return Err(UpdateError::Unknown);
        }
        self.regions
            .lock()
            .or(Err(UpdateError::Unknown))
            .and_then(|mut locked| {
                if let Some(index) = locked.iter().position(|v| &v.id == &id) {
                    let updated = Region { id, name };
                    if locked
                        .iter()
                        .find(|v| &v.id != &updated.id && &v.name == &updated.name)
                        .is_some()
                    {
                        return Err(UpdateError::DuplicatedName);
                    }
                    locked.splice(index..index + 1, vec![updated]);
                    Ok(())
                } else {
                    Err(UpdateError::NotFound)
                }
            })
    }
}
