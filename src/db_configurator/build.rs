use crate::db_configurator::parser::DBConfiguration;

pub fn build_plan(configuration: DBConfiguration) -> Plan {
    let mut plan = Plan {
        year_plans: vec![],
        score_plan: vec![],
    };
    let config = &configuration;

    for year in config.years.iter() {
        let year_id = year.id.clone();
        let year_name = year.name.clone();

        let mut year_plan = YearPlan {
            id: year_id.clone(),
            name: year_name,
            forms: vec![],
            events: vec![],
        };

        for form in config.forms.iter() {
            if configuration.is_year_applicable_to_form(form, year_id.clone()) {
                let form_plan = FormPlan {
                    id: format!("{}-{}", year_plan.clone().id, form.clone().id),
                    name: form.clone().name,
                };
                year_plan.forms.push(form_plan.clone());
            }
        }
        for event in config.events.iter() {
            if !configuration.is_event_applicable_to_year(event, &year.clone().id) {
                continue;
            }
            let mut applies_to_at_least_one_form = false;
            for form in config.forms.iter() {
                if configuration.is_year_applicable_to_form(form, year_id.clone())
                    && configuration.is_event_applicable_to_form(event, form.clone().id)
                {
                    applies_to_at_least_one_form = true;
                    break;
                }
            }

            if !applies_to_at_least_one_form {
                continue;
            }

            for gender in config.genders.iter() {
                if configuration.is_event_applicable_to_gender(event, gender) {
                    year_plan.events.push(EventPlan {
                        id: format!("{}-{}-{}", year_plan.clone().id, gender, event.clone().id),
                        name: event.clone().name,
                        gender_id: gender.clone(),
                    })
                }
            }
        }
        plan.year_plans.push(year_plan);
    }
    for score in config.scores.iter() {
        plan.score_plan.push(ScorePlan {
            name: score.clone().name,
            value: score.clone().value,
        })
    }
    plan
}

#[derive(Debug)]
pub struct Plan {
    pub year_plans: Vec<YearPlan>,
    pub score_plan: Vec<ScorePlan>,
}

#[derive(Debug)]
pub struct ScorePlan {
    pub name: String,
    pub value: i64,
}

#[derive(Debug, Clone)]

pub struct YearPlan {
    pub id: String,
    pub name: String,
    pub forms: Vec<FormPlan>,
    pub events: Vec<EventPlan>,
}

#[derive(Debug, Clone)]

pub struct FormPlan {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone)]

pub struct EventPlan {
    pub id: String,
    pub name: String,
    pub gender_id: String,
}
