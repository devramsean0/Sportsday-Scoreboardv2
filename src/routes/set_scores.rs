use actix_web::{get, web, HttpResponse};
use askama::Template;

use crate::{db::events::Events, templates::SetScoresTemplate, AppState};

#[get("/set_scores")]
pub async fn get(state: web::Data<AppState>) -> HttpResponse {
    let events = Events::all(&state.pool).await.unwrap();
    HttpResponse::Ok().body(
        SetScoresTemplate {
            events,
            forms: state.config.forms.clone(),
            scores: state.config.scores.clone(),
        }
        .render()
        .expect("Template should be valid"),
    )
}
