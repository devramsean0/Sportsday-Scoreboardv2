use async_sqlite::Pool;

pub mod events;
pub mod user_sessions;
pub mod users;
pub mod years;

pub async fn create_tables(pool: &Pool) -> Result<(), async_sqlite::Error> {
    pool.conn(move |conn| {
        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS years (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL
            );",
            [],
        )
        .unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                year_id TEXT NOT NULL,
                gender_id TEXT NOT NULL,
                filter_key TEXT NOT NULL,
                scores TEXT NOT NULL DEFAULT '{}',
                FOREIGN KEY (year_id) REFERENCES years(id)
            );",
            [],
        )
        .unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                email STRING UNIQUE NOT NULL,
                has_admin INT NOT NULL DEFAULT 0,
                has_set_score INT NOT NULL DEFAULT 0
            );",
            [],
        )
        .unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS user_sessions (
                    id TEXT PRIMARY KEY,
                    user_id INTEGER NOT NULL,
                    has_admin INTEGER NOT NULL DEFAULT 0,
                    has_set_score INTEGER NOT NULL DEFAULT 0,
                    FOREIGN KEY (user_id) REFERENCES users(id)
            );",
            [],
        )
        .unwrap();
        Ok(())
    })
    .await?;
    Ok(())
}
