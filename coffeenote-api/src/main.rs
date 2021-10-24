use tonic::{
    transport::{Identity, Server, ServerTlsConfig},
    Request, Response, Status,
};

use hello_world::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};
mod hello_world {
    tonic::include_proto!("helloworld");
}

mod infrastructures;
mod result;

use infrastructures::database;
use result::{CoffeeNoteResult, Error};
use std::env;

#[derive(Debug, Default)]
struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };
        Ok(Response::new(reply))
    }
}

pub fn read_env(key: &str) -> CoffeeNoteResult<String> {
    env::var(key).map_err(|_| Error::MissingEnvVar(key.to_owned()))
}

#[tokio::main]
async fn main() -> CoffeeNoteResult<()> {
    let dbinfo = database::postgres::DbInfo::from_env()?;
    let cert = tokio::fs::read("./coffeenote-api/tls/cert.pem")
        .await
        .map_err(|_| Error::TlsCertNotFound)?;
    let key = tokio::fs::read("./coffeenote-api/tls/key.pem")
        .await
        .map_err(|_| Error::TlsKeyNotFound)?;
    let identity = Identity::from_pem(cert, key);
    let pg = database::postgres::connect(3, &dbinfo).await;
    let addr = "0.0.0.0:55301"
        .parse()
        .map_err(|_| Error::CannotParseAddressString)?;
    let greeter = MyGreeter::default();
    Server::builder()
        .tls_config(ServerTlsConfig::new().identity(identity))
        .map_err(|_| Error::TlsConfigError)?
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await
        .map_err(Error::CannotStartServer)?;
    Ok(())
}
