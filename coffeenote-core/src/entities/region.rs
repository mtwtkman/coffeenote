use std::ops::Deref;
use uuid::Uuid;
use super::{CoffeeNoteEntity, CoffeeNoteValueObject, ValidationResult, Validate, Invalid, Valid};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionId(Uuid);
impl CoffeeNoteValueObject for RegionId {}
impl From<Uuid> for RegionId {
    fn from(v: Uuid) -> Self {
        Self(v)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionName(String);
impl CoffeeNoteValueObject for RegionName {}
impl From<String> for RegionName {
    fn from(v: String) -> Self {
        Self(v)
    }
}
impl RegionName {
    const fn max_length() -> usize { 256 }
    const fn min_length() -> usize { 1 }
}
impl Validate for RegionName {
    type ValueType = String;
    fn validate(&self) -> ValidationResult<Self::ValueType> {
        let length = self.0.len();
        if length < Self::min_length() {
            return Err(Invalid(self.0.clone()));
        } else if length > Self::max_length() {
            return Err(Invalid(self.0.clone()));
        }
        return Ok(Valid)
    }
}
impl Deref for RegionName {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    pub id: RegionId,
    pub name: RegionName,
}
impl CoffeeNoteEntity for Region {}
impl Region {
    pub fn new(name: String) -> Self {
        let id = Uuid::new_v4();
        Self {
            id: RegionId::from(id),
            name: RegionName::from(name),
        }
    }
}