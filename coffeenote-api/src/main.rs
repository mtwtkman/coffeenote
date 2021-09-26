mod infrastructures;
mod result;

use infrastructures::database;
use result::{CoffeeNoteResult, Error};
use std::env;
use warp::Filter;

pub fn read_env(key: &str) -> CoffeeNoteResult<String> {
    env::var(key).map_err(|_| Error::MissingEnvVar(key.to_owned()))
}

#[tokio::main]
async fn main() -> CoffeeNoteResult<()> {
    let dbinfo = database::postgres::DbInfo::from_env()?;
    let pg = database::postgres::connect(3, &dbinfo).await;
    let hello = warp::path!("hi").map(|| format!("hi yo"));
    warp::serve(hello)
        .tls()
        .cert_path("coffeenote-api/tls/cert.pem")
        .key_path("coffeenote-api/tls/key.pem")
        .run(([0, 0, 0, 0], 55301))
        .await;
    Ok(())
}
