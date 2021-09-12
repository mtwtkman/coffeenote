use uuid::Uuid;

use super::region::Region;

pub struct ProductionArea {
    pub id: Uuid,
    pub name: String,
    pub region: Region,
}

impl ProductionArea {
    pub fn new(name: String, region: Region) -> Self {
        let id = Uuid::new_v4();
        Self { id, name, region }
    }
}
