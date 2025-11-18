use async_sqlite::{rusqlite::Row, Pool};

#[derive(Clone)]
pub struct Forms {
    pub id: String,
    pub name: String,
}

impl Forms {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }

    fn map_from_row(row: &Row) -> Result<Self, async_sqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }

    pub async fn insert(self, pool: &Pool) -> Result<(), async_sqlite::Error> {
        pool.conn(move |conn| {
            conn.execute(
                "INSERT INTO forms(id, name) VALUES (?1, ?2);",
                [self.id, self.name],
            )
            .unwrap();
            Ok(())
        })
        .await?;
        Ok(())
    }

    pub async fn all(pool: &Pool) -> Result<Vec<Self>, async_sqlite::Error> {
        pool.conn(move |conn| {
            let mut stmt = conn.prepare("SELECT * FROM forms")?;
            let form_iter = stmt
                .query_map([], |row| Ok(Self::map_from_row(row).unwrap()))
                .unwrap();
            let mut forms = Vec::new();

            for form in form_iter {
                forms.push(form?);
            }
            Ok(forms)
        })
        .await
    }
}
