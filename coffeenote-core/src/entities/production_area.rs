use uuid::Uuid;

use super::{region::Region, CoffeeNoteEntity, CoffeeNoteValueObject};

pub struct ProductionAreaId(Uuid);
impl CoffeeNoteValueObject for ProductionAreaId {}
impl From<Uuid> for ProductionAreaId {
    fn from(v: Uuid) -> Self {
        Self(v)
    }
}

pub struct ProductionAreaName(String);
impl CoffeeNoteValueObject for ProductionAreaName {}
impl From<String> for ProductionAreaName {
    fn from(v: String) -> Self {
        Self(v)
    }
}

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
