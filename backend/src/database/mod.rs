use tokio_rusqlite::{Connection, Result};

pub struct Database {
    pub conn: Connection
}
impl Database {
    pub fn from(conn: Connection) -> Self {
        Self {conn}
    }
}

pub async fn create_connection() -> Result<Database> {
    // let conn = Connection::open("database.db")?;
    let conn = Connection::open_in_memory().await?;

    conn.call(|conn| {
        conn.execute(
            "CREATE TABLE users (
                id   INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                password TEXT NOT NULL,
                data BLOB
            )",
            (),
        )?;

        Ok(())  
    })
    .await?;

    Ok(Database::from(conn))
}
