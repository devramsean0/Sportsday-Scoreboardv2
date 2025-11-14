use async_sqlite::Pool;
use log::debug;

#[derive(Clone)]
pub struct Events {
    id: String,
    name: String,
    year_id: String,
    gender_id: String,
}

impl Events {
    pub fn new(id: String, name: String, year_id: String, gender_id: String) -> Self {
        Self {
            id,
            name,
            year_id,
            gender_id,
        }
    }

    pub async fn insert(self, pool: &Pool) -> Result<(), async_sqlite::Error> {
        pool.conn(move |conn| {
            debug!("Inserting Event with id {}", self.id);
            conn.execute(
                "INSERT INTO events(id, name, year_id, gender_id) VALUES (?1, ?2, ?3, ?4);",
                [self.id, self.name, self.year_id, self.gender_id],
            )
            .unwrap();
            Ok(())
        })
        .await?;
        Ok(())
    }
}
