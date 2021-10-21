use uuid::Uuid;

use super::{
    production_area::ProductionArea, shop::Shop, CoffeeNoteEntity, CoffeeNoteValueObject, Datetime,
};

pub struct BeanId(Uuid);
impl CoffeeNoteValueObject for BeanId {}
impl From<Uuid> for BeanId {
    fn from(v: Uuid) -> Self {
        Self(v)
    }
}

#[derive(Debug)]
pub struct BeanName(String);
impl CoffeeNoteValueObject for BeanName {}
impl From<String> for BeanName {
    fn from(v: String) -> Self {
        Self(v)
    }
}

#[derive(Debug)]
pub struct BeanPurchasedAt(Datetime);
impl CoffeeNoteValueObject for BeanPurchasedAt {}
impl From<Datetime> for BeanPurchasedAt {
    fn from(v: Datetime) -> Self {
        Self(v)
    }
}

pub struct Bean {
    pub id: BeanId,
    pub name: BeanName,
    pub shop: Shop,
    pub production_area: ProductionArea,
    pub purchased_at: BeanPurchasedAt,
}
impl CoffeeNoteEntity for Bean {}
impl Bean {
    pub fn new(
        name: String,
        shop: Shop,
        production_area: ProductionArea,
        purchased_at: Datetime,
    ) -> Self {
        let id = Uuid::new_v4();
        Self {
            id: BeanId::from(id),
            name: BeanName::from(name),
            shop,
            production_area,
            purchased_at: BeanPurchasedAt::from(purchased_at),
        }
    }
}
