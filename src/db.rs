use anyhow::Context;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> anyhow::Result<PgConnection> {
    let database_url = env::var("DATABASE_URL")?;
    PgConnection::establish(&database_url)
        .context(format!("Failed to connect {}", database_url))
}
