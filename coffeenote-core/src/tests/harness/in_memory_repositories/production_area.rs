use crate::entities::production_area::{ProductionArea, ProductionAreaId};
use crate::repositories::production_area::{
    CreateError, DeleteError, FetchAllError, FetchOneError, NewProductionArea,
    ProductionAreaRepository,
};
use std::sync::Mutex;

pub struct InMemory {
    production_areas: Mutex<Vec<ProductionArea>>,
    error: bool,
}

impl InMemory {
    fn new(production_areas: Vec<ProductionArea>, error: bool) -> Self {
        Self {
            production_areas: Mutex::new(production_areas),
            error,
        }
    }
}

impl ProductionAreaRepository for InMemory {
    fn fetch_one(&self, id: ProductionAreaId) -> Result<ProductionArea, FetchOneError> {
        if self.error {
            return Err(FetchOneError::Unknown);
        }
        self.production_areas
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
    fn fetch_all(&self) -> Result<Vec<ProductionArea>, FetchAllError> {
        if self.error {
            return Err(FetchAllError::Unknown);
        }
        self.production_areas
            .lock()
            .or(Err(FetchAllError::Unknown))
            .map(|locked| locked.clone())
    }
    fn create(&self, value: NewProductionArea) -> Result<ProductionArea, CreateError> {
        if self.error {
            return Err(CreateError::Unknown);
        }
        self.production_areas
            .lock()
            .or(Err(CreateError::Unknown))
            .and_then(|mut locked| {
                if let Some(_found) = locked.iter().find(|v| &*v.name == &value.name) {
                    return Err(CreateError::DuplicatedName);
                }
                let new_one = ProductionArea::new(value.name, value.region);
                locked.push(new_one.clone());
                Ok(new_one)
            })
    }
    fn delete(
        &self,
        id: ProductionAreaId,
    ) -> Result<(), crate::repositories::production_area::DeleteError> {
        if self.error {
            return Err(DeleteError::Unknown);
        }
        self.production_areas
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
