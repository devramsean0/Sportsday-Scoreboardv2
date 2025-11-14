use async_sqlite::Pool;

#[derive(Clone)]
pub struct Forms {
    id: String,
    name: String,
    year_id: String,
}

impl Forms {
    pub fn new(id: String, name: String, year_id: String) -> Self {
        Self { id, name, year_id }
    }

    pub async fn insert(self, pool: &Pool) -> Result<(), async_sqlite::Error> {
        pool.conn(move |conn| {
            conn.execute(
                "INSERT INTO forms(id, name, year_id) VALUES (?1, ?2, ?3);",
                [self.id, self.name, self.year_id],
            )
            .unwrap();
            Ok(())
        })
        .await?;
        Ok(())
    }
}
