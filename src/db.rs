use sqlx::{Pool, Mssql};
use std::env;

/// Represents a connection pool to a Microsoft SQL Server database.
pub struct DbPool {
    pub pool: Pool<Mssql>,
}

impl DbPool {
    /// Creates a new `DbPool` instance.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    /// Checks that if DATABASE_URL does not exist, `DbPool::new()` fails.
    #[actix_web::test]
    async fn test_dbpool_new_missing_env() {
        // We remove the variable to simulate that it is not defined
        env::remove_var("DATABASE_URL");

        // We call DbPool::new()
        let result = DbPool::new().await;
        // We hope this is an error, as there is no DATABASE_URL
        assert!(result.is_err(),"Should have failed when DATABASE_URL is not defined");
    }

    /// Verify that with a valid URL and the DB active, it connects.
    /// For this to work, you must have your MSSQL container running
    /// and a test DB at the URL you enter.
    #[actix_web::test]
    async fn test_dbpool_new_valid() {
        // Adjust user, password, host, etc. to your environment
        env::set_var("DATABASE_URL", "mssql://sa:Tester*31@localhost:1433/master");

        let result = DbPool::new().await;
        assert!(result.is_ok(),"You should have successfully connected with a valid URL and the DB running");
    }
}