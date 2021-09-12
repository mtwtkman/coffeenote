use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{production_area::ProductionArea, shop::Shop};

pub struct Bean {
    pub id: Uuid,
    pub name: String,
    pub shop: Shop,
    pub production_area: ProductionArea,
    pub purchased_at: DateTime<Utc>,
}

impl Bean {
    pub fn new(
        name: String,
        shop: Shop,
        production_area: ProductionArea,
        purchased_at: DateTime<Utc>,
    ) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            name,
            shop,
            production_area,
            purchased_at,
        }
    }
}
