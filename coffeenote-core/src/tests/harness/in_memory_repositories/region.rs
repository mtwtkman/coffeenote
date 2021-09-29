use crate::entities::region::{Region, RegionId};
use crate::repositories::region::{
    CreateError, DeleteError, FetchAllError, FetchOneError, NewRegion, RegionRepository,
};
use async_trait::async_trait;
use futures::lock::Mutex;

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

#[async_trait]
impl RegionRepository for InMemory {
    async fn fetch_one(&self, id: RegionId) -> Result<Region, FetchOneError> {
        if self.error {
            return Err(FetchOneError::Unknown);
        }
        let locked = self.regions.lock().await;
        if let Some(v) = locked.iter().find(|v| &v.id == &id) {
            Ok(v.clone())
        } else {
            Err(FetchOneError::NotFound)
        }
    }
    async fn fetch_all(&self) -> Result<Vec<Region>, FetchAllError> {
        if self.error {
            return Err(FetchAllError::Unknown);
        }
        let locked = self.regions.lock().await;
        Ok(locked.clone())
    }
    async fn create(&self, value: NewRegion) -> Result<Region, CreateError> {
        if self.error {
            return Err(CreateError::Unknown);
        }
        let mut locked = self.regions.lock().await;
        if let Some(_found) = locked.iter().find(|v| &*v.name == &value.name) {
            return Err(CreateError::DuplicatedName);
        }
        let new_one = Region::new(value.name);
        locked.push(new_one.clone());
        Ok(new_one)
    }
    async fn delete(&self, id: RegionId) -> Result<(), DeleteError> {
        if self.error {
            return Err(DeleteError::Unknown);
        }
        let mut locked = self.regions.lock().await;
        if let Some(index) = locked.iter().position(|v| &v.id == &id) {
            locked.remove(index);
            Ok(())
        } else {
            Err(DeleteError::NotFound)
        }
    }
}
