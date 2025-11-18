use askama::Template;
use std::collections::HashMap;

use crate::db::{forms::Forms, years::Years};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "scoreboard.html")]
pub struct ScoreboardTemplate {
    pub forms: Vec<Forms>,
    pub years: Vec<Years>,
    pub scores: HashMap<String, HashMap<String, i64>>,
    pub year_totals: HashMap<String, i64>,
    pub form_totals: HashMap<String, i64>,
    pub grand_total: i64,
}
