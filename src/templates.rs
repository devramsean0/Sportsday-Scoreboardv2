use askama::Template;
use std::collections::HashMap;

use crate::{configurator::parser::Form, db::years::Years};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "scoreboard.html")]
pub struct ScoreboardTemplate {
    pub forms: Vec<Form>,
    pub years: Vec<Years>,
    pub scores: HashMap<String, HashMap<String, i64>>,
    pub year_totals: HashMap<String, i64>,
    pub form_totals: HashMap<String, i64>,
    pub grand_total: i64,
}
