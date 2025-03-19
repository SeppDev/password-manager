use std::path::Path;
use tokio_rusqlite::{Connection, Result};

mod accounts;
mod init;

pub struct Database {
    pub conn: Connection,
}
impl Database {
    pub fn from(conn: Connection) -> Self {
        Self { conn }
    }
    pub async fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self::from(Connection::open(path).await?))
    }
    pub async fn new_memory() -> Result<Self> {
        Ok(Self::from(Connection::open_in_memory().await?))
    }
}
