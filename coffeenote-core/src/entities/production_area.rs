use std::ops::Deref;
use uuid::Uuid;

use super::{
    region::Region, CoffeeNoteEntity, CoffeeNoteValueObject, Invalid, Valid, Validate,
    ValidationResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductionAreaId(Uuid);
impl CoffeeNoteValueObject for ProductionAreaId {}
impl From<Uuid> for ProductionAreaId {
    fn from(v: Uuid) -> Self {
        Self(v)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductionAreaName(String);
impl CoffeeNoteValueObject for ProductionAreaName {}
impl ProductionAreaName {
    const fn min_len() -> usize {
        1
    }
    const fn max_len() -> usize {
        256
    }
}
impl Validate for ProductionAreaName {
    type ValueType = String;
    fn validate(&self) -> ValidationResult<Self::ValueType> {
        let length = self.0.len();
        if length < Self::min_len() {
            return Err(Invalid(self.0.clone()));
        } else if length > Self::max_len() {
            return Err(Invalid(self.0.clone()));
        }
        return Ok(Valid);
    }
}
impl From<String> for ProductionAreaName {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl Deref for ProductionAreaName {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductionArea {
    pub id: ProductionAreaId,
    pub name: ProductionAreaName,
    pub region: Region,
}
impl CoffeeNoteEntity for ProductionArea {}
impl ProductionArea {
    pub fn new(name: String, region: Region) -> Self {
        let id = Uuid::new_v4();
        Self {
            id: ProductionAreaId::from(id),
            name: ProductionAreaName::from(name),
            region,
        }
    }
}
