use std::env;

use sqlx::postgres::PgPoolOptions;

pub async fn setup_database() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in environment");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
