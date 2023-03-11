use anyhow::Context;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> anyhow::Result<PgConnection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    PgConnection::establish(&database_url)
        .context(format!("Failed to connect {}", database_url))
}

#[cfg(test)]
mod databse_tests {
    use super::*;

    #[test]
    fn db_conn() {
        establish_connection()
            .expect("Failed to connect database");
    }
}
