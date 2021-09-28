pub mod bean;
pub mod production_area;
pub mod region;
pub mod shop;

pub trait CoffeeNoteEntity {}
pub trait CoffeeNoteValueObject {}


#[derive(Debug)]
pub struct Invalid<T>(T);

#[derive(Debug)]
pub struct Valid;

pub type ValidationResult<T> = Result<Valid, Invalid<T>>;

pub trait Validate {
    type ValueType;
    fn validate(&self) -> ValidationResult<Self::ValueType>;
    fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}