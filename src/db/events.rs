use async_sqlite::{rusqlite::Row, Pool};
use log::debug;
use serde_json::Value;

#[derive(Clone)]
pub struct Events {
    pub id: String,
    pub name: String,
    pub year_id: String,
    pub gender_id: String,
    pub scores: String,
}

impl Events {
    pub fn new(
        id: String,
        name: String,
        year_id: String,
        gender_id: String,
        scores: String,
    ) -> Self {
        Self {
            id,
            name,
            year_id,
            gender_id,
            scores: scores,
        }
    }

    fn map_from_row(row: &Row) -> Result<Self, async_sqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
            year_id: row.get(2)?,
            gender_id: row.get(3)?,
            scores: row.get(4)?,
        })
    }

    pub async fn insert(self, pool: &Pool) -> Result<(), async_sqlite::Error> {
        pool.conn(move |conn| {
            debug!("Inserting Event with id {}", self.id);
            conn.execute(
                "INSERT INTO events(id, name, year_id, gender_id, scores) VALUES (?1, ?2, ?3, ?4, ?5);",
                [self.id, self.name, self.year_id, self.gender_id, self.scores],
            )
            .unwrap();
            Ok(())
        })
        .await?;
        Ok(())
    }

    pub async fn all(pool: &Pool) -> Result<Vec<Self>, async_sqlite::Error> {
        pool.conn(move |conn| {
            let mut stmt = conn.prepare("SELECT * FROM events")?;
            let event_iter = stmt
                .query_map([], |row| Ok(Self::map_from_row(row).unwrap()))
                .unwrap();
            let mut events = Vec::new();

            for event in event_iter {
                events.push(event?);
            }
            Ok(events)
        })
        .await
    }

    pub async fn set_scores(
        pool: &Pool,
        id: String,
        scores: Value,
    ) -> Result<(), async_sqlite::Error> {
        pool.conn(move |conn| {
            debug!("Setting Scores for Event with id {}", id);
            conn.execute(
                "UPDATE events SET scores = ?1 WHERE id = ?2;",
                [serde_json::to_string(&scores).unwrap(), id],
            )
            .unwrap();
            Ok(())
        })
        .await?;
        Ok(())
    }
}
