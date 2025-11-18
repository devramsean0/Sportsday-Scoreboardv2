use actix_web::{get, web, HttpResponse};
use askama::Template;

use crate::{templates::IndexTemplate, AppState};

#[get("/")]
pub async fn get(_state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().body(IndexTemplate {}.render().expect("Template should be valid"))
}
