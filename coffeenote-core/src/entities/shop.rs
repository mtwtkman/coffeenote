use uuid::Uuid;

use super::{CoffeeNoteEntity, CoffeeNoteValueObject};

pub struct ShopId(Uuid);
impl CoffeeNoteValueObject for ShopId {}
impl From<Uuid> for ShopId {
    fn from(v: Uuid) -> Self {
        Self(v)
    }
}

pub struct ShopName(String);
impl CoffeeNoteValueObject for ShopName {}
impl From<String> for ShopName {
    fn from(v: String) -> Self {
        Self(v)
    }
}

pub struct ShopUrl(String);
impl CoffeeNoteValueObject for ShopUrl {}
impl From<String> for ShopUrl {
    fn from(v: String) -> Self {
        Self(v)
    }
}

pub struct Shop {
    pub id: ShopId,
    pub name: ShopName,
    pub url: ShopUrl,
}
impl CoffeeNoteEntity for Shop {}
impl Shop {
    pub fn new(name: String, url: String) -> Self {
        let id = Uuid::new_v4();
        Self {
            id: ShopId::from(id),
            name: ShopName::from(name),
            url: ShopUrl::from(url),
        }
    }
}
