use async_sqlite::Pool;

use crate::db::{events::Events, forms::Forms};

#[derive(Clone)]
pub struct Years {
    id: String,
    name: String,
    forms: Vec<Forms>,
    events: Vec<Events>, // TODO: Events as optionals
}

impl Years {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            forms: vec![],
            events: vec![],
        }
    }

    pub async fn insert(self, pool: &Pool) -> Result<Self, async_sqlite::Error> {
        let id = self.id.clone();
        let name = self.name.clone();
        pool.conn(move |conn| {
            conn.execute("INSERT INTO years(id, name) VALUES (?1, ?2);", [id, name])
                .unwrap();
            Ok(())
        })
        .await?;
        Ok(self)
    }

    pub async fn new_form(
        mut self,
        pool: &Pool,
        id: String,
        name: String,
    ) -> Result<Self, async_sqlite::Error> {
        let form = Forms::new(id, name, self.clone().id);
        self.forms.push(form.clone());
        form.insert(&pool).await?;

        Ok(self)
    }

    pub async fn new_event(
        mut self,
        pool: &Pool,
        id: String,
        name: String,
        gender_id: String,
    ) -> Result<Self, async_sqlite::Error> {
        let event = Events::new(id, name, self.clone().id, gender_id);
        self.events.push(event.clone());
        event.insert(&pool).await?;

        Ok(self)
    }
}
