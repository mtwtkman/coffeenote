use crate::entities::{
    bean::{Bean, BeanId, BeanName, BeanPurchasedAt},
    production_area::ProductionArea,
    shop::Shop,
};
use crate::repositories::CoffeeNoteRepository;
use chrono::{DateTime, Utc};

#[derive(Debug)]
enum FetchOneError {
    Unknown,
    NotFound,
}

#[derive(Debug)]
enum FetchAllError {
    Unknown,
}

#[derive(Debug)]
enum CreateError {
    Unknown,
    DupulicatedName,
}

#[derive(Debug)]
enum DeleteError {
    Unknown,
    NotFound,
}

pub struct CreateBean {
    name: BeanName,
    purchased_at: BeanPurchasedAt,
    shop: Shop,
    production_area: ProductionArea,
}

impl CreateBean {
    pub fn new(
        name: String,
        purchased_at: DateTime<Utc>,
        shop: Shop,
        production_area: ProductionArea,
    ) -> Self {
        Self {
            name: BeanName::from(name),
            purchased_at: BeanPurchasedAt::from(purchased_at),
            shop,
            production_area,
        }
    }
}

trait BeanRepository: CoffeeNoteRepository<Bean> {
    fn fetch_one(&self, id: BeanId) -> Result<Bean, FetchOneError>;
    fn fetch_all(&self) -> Result<Vec<Bean>, FetchAllError>;
    fn create(&self, value: CreateBean) -> Result<Bean, CreateError>;
    fn delete(&self, id: BeanId) -> Result<(), DeleteError>;
}
