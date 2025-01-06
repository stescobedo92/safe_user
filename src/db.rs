use sqlx::{Pool, Mssql};
use std::env;

/// Represents a connection pool to a Microsoft SQL Server database.
pub struct DbPool {
    pub pool: Pool<Mssql>,
}

impl DbPool {
    /// Creates a new `DbPool` instance.
    ///
    /// # Returns
    ///
    /// * `Ok(DbPool)` - If the connection to the database is successful.
    /// * `Err(sqlx::Error)` - If there is an error in retrieving the `DATABASE_URL` environment variable or connecting to the database.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The `DATABASE_URL` environment variable is not set.
    /// * There is an error connecting to the database.
    pub async fn new() -> Result<Self, sqlx::Error> {
        // En lugar de .expect(), hacemos un match para devolver Err
        let database_url = match env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(e) => {
                return Err(sqlx::Error::Configuration(Box::new(e)));
            }
        };

        let pool = sqlx::mssql::MssqlPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(DbPool { pool })
    }
}