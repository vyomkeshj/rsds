/**
*   Runs the rsds-logserver
*   Logserver exposes an api that the workers and scheduler use to send their logs
*
*
*/
use sqlx::Connection;

#[async_std::main]
// or #[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let conn = SqliteConnection::connect("sqlite::memory:").await?;

    // Make a simple query to return the given parameter
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&conn).await?;

    assert_eq!(row.0, 150);

    Ok(())
}
