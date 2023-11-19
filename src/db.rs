use std::env;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Result};

pub async fn get_db_pool() -> Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await?;

    Ok(pool)
}
