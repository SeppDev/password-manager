use super::Database;

impl Database {
    pub async fn init_connection(&self) {
        self.conn
            .call(|conn| {
                // conn.execute(
                //     "CREATE TABLE users (
                //         id   INTEGER PRIMARY KEY,
                //         name TEXT NOT NULL,
                //         password TEXT NOT NULL,
                //         data BLOB
                //     )",
                //     (),
                // )?;

                conn.execute(
                    "CREATE TABLE IF NOT EXISTS users (
                id   INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL,
                data BLOB
            )",
                    (),
                )?;

                conn.execute(
                    "CREATE TABLE IF NOT EXISTS sessions (
                token TEXT NOT NULL PRIMARY KEY,
                expires_at DATETIME NOT NULL,
                user_id INTEGER NOT NULL,
                FOREIGN KEY (user_id) REFERENCES users(id)
            )",
                    (),
                )?;

                Ok(())
            })
            .await
            .expect("Failed to initialize database");
    }
}
