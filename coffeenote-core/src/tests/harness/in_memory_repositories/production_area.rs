use futures::lock::Mutex;
use async_trait::async_trait;
use crate::repositories::production_area::{CreateError, DeleteError, FetchAllError, FetchOneError, NewProductionArea, ProductionAreaRepository};
use crate::entities::production_area::{ProductionArea, ProductionAreaId};

pub struct InMemory {
    production_areas: Mutex<Vec<ProductionArea>>,
    error: bool,
}

impl InMemory {
    fn new(production_areas: Vec<ProductionArea>, error: bool) -> Self {
        Self { production_areas: Mutex::new(production_areas), error }
    }
}

#[async_trait]
impl ProductionAreaRepository for InMemory {
    async fn fetch_one(&self, id: ProductionAreaId) -> Result<ProductionArea, FetchOneError> {
        if self.error {
            return Err(FetchOneError::Unknown);
        }
        let locked = self.production_areas.lock().await;
        if let Some(v) = locked.iter().find(|v| &v.id == &id) {
            Ok(v.clone())
        } else {
            Err(FetchOneError::NotFound)
        }
    }

    async fn fetch_all(&self) -> Result<Vec<ProductionArea>, FetchAllError> {
        if self.error {
            return Err(FetchAllError::Unknown);
        }
        let locked = self.production_areas.lock().await;
        Ok(locked.clone())
    }

    async fn create(&self, value: NewProductionArea) -> Result<ProductionArea, CreateError> {
        if self.error {
            return Err(CreateError::Unknown);
        }
        let mut locked = self.production_areas.lock().await;
        if let Some(_found) = locked.iter().find(|v| &*v.name == &value.name) {
            return Err(CreateError::DuplicatedName);
        }
        let new_one = ProductionArea::new(value.name, value.region);
        locked.push(new_one.clone());
        Ok(new_one)
    }

    async fn delete(&self, id: ProductionAreaId) -> Result<(), crate::repositories::production_area::DeleteError> {
        if self.error {
            return Err(DeleteError::Unknown);
        }
        let mut locked = self.production_areas.lock().await;
        if let Some(index) = locked.iter().position(|v| &v.id == &id) {
            locked.remove(index);
            Ok(())
        } else {
            Err(DeleteError::NotFound)
        }
    }
}