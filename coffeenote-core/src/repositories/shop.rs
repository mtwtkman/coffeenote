use crate::entities::shop::{Shop, ShopId, ShopName, ShopUrl};

#[derive(Debug)]
pub enum FetchOneError {
    Unknown,
    NotFound,
}

#[derive(Debug)]
pub enum FetchAllError {
    Unknown,
}

#[derive(Debug)]
pub enum CreateError {
    Unknown,
    DuplicatedName,
}

#[derive(Debug)]
pub enum DeleteError {
    Unknown,
    NotFound,
}

pub struct CreateShop {
    name: ShopName,
    url: ShopUrl,
}

impl CreateShop {
    pub fn new(name: String, url: String) -> Self {
        Self {
            name: ShopName::from(name),
            url: ShopUrl::from(url),
        }
    }
}

pub trait ShopRepository {
    fn fetch_one(&self, id: ShopId) -> Result<Shop, FetchOneError>;
    fn fetch_all(&self) -> Result<Vec<Shop>, FetchAllError>;
    fn create(&self, value: CreateShop) -> Result<Shop, CreateError>;
    fn delete(&self, id: ShopId) -> Result<(), DeleteError>;
}
