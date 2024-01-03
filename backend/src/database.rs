use std::env;

use sqlx::postgres::PgPoolOptions;

pub fn setup_database() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in environment");

    // connecting lazily because it takes time for
    // database to be available on the cloud
    PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&database_url)
}
