pub type CoffeeNoteResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MissingEnvVar(String),
    TlsCertNotFound,
    TlsKeyNotFound,
    TlsConfigError,
    CannotParseAddressString,
    CannotStartServer(tonic::transport::Error),
}
