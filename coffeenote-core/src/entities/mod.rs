pub mod bean;
pub mod production_area;
pub mod region;
pub mod shop;

pub trait CoffeeNoteEntity {}
pub trait CoffeeNoteValueObject {}


pub struct Invalid<T>(T);
pub struct Valid;
pub trait Validate {
    type ValueType;
    fn validate(&self) -> Result<Valid, Invalid<Self::ValueType>>;
    fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}