use uuid::Uuid;

use super::{CoffeeNoteEntity, CoffeeNoteValueObject};

pub struct RegionId(Uuid);
impl CoffeeNoteValueObject for RegionId {}
impl From<Uuid> for RegionId {
    fn from(v: Uuid) -> Self {
        Self(v)
    }
}

pub struct RegionName(String);
impl CoffeeNoteValueObject for RegionName {}
impl From<String> for RegionName {
    fn from(v: String) -> Self {
        Self(v)
    }
}

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
