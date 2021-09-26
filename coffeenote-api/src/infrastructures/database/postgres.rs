use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use super::Database;
use crate::{read_env, result::CoffeeNoteResult};

pub struct DbInfo {
    user: String,
    password: String,
    db_name: String,
}

impl DbInfo {
    pub fn from_env() -> CoffeeNoteResult<Self> {
        Ok(Self {
            user: read_env("POSTGRES_USER")?,
            password: read_env("POSTGRES_PASSWORD")?,
            db_name: read_env("POSTGRES_DB")?,
        })
    }

    pub fn build_uri(&self) -> String {
        format!(
            "postgres://{}:{}@{}",
            self.user, self.password, self.db_name
        )
    }
}

pub async fn connect<'a>(
    max_connections: u32,
    dbinfo: &DbInfo,
) -> Result<Database<Pool<Postgres>>, sqlx::Error> {
    let client = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&dbinfo.build_uri())
        .await?;
    Ok(Database { client })
}
