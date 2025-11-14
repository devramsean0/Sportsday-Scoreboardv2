use async_sqlite::Pool;

pub struct Scores {
    name: String,
    value: i64,
}

impl Scores {
    pub fn new(name: String, value: i64) -> Self {
        Self { name, value }
    }

    pub async fn insert(self, pool: &Pool) -> Result<(), async_sqlite::Error> {
        pool.conn(move |conn| {
            conn.execute(
                "INSERT INTO scores(name, value) VALUES (?1, ?2);",
                [self.name, self.value.to_string()],
            )
            .unwrap();
            Ok(())
        })
        .await?;
        Ok(())
    }
}
