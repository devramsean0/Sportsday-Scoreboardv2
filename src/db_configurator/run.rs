use async_sqlite::Pool;
use log::{debug, info};

use crate::{
    db::{scores::Scores, years::Years},
    db_configurator::build::Plan,
};

pub async fn run(plan: Plan, pool: &Pool) -> Result<(), async_sqlite::Error> {
    info!("Implementing Plan");
    for score in plan.score_plan.iter() {
        Scores::new(score.name.clone(), score.value)
            .insert(&pool)
            .await?;
    }

    for year in plan.year_plans.iter() {
        debug!("Inserting Planned Year {}", year.id);
        let mut year_struct = Years::new(year.id.clone(), year.name.clone())
            .insert(&pool)
            .await?;
        for form in year.forms.iter() {
            debug!("Inserting Planned Form {}", form.id);
            year_struct = year_struct
                .new_form(&pool, form.clone().id, form.clone().name)
                .await?;
        }
        for event in year.events.iter() {
            debug!("Inserting Planned Event {}", event.id);
            year_struct = year_struct
                .new_event(
                    &pool,
                    event.clone().id,
                    event.clone().name,
                    event.clone().gender_id,
                )
                .await?
        }
    }
    Ok(())
}
