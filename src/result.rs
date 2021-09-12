pub type CoffeeNoteResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MissingEnvVar(String),
}
