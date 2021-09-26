pub mod bean;
pub mod production_area;
pub mod region;
pub mod shop;

use crate::entities::CoffeeNoteEntity;

pub trait CoffeeNoteRepository<T: CoffeeNoteEntity> {}
