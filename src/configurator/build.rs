use crate::configurator::parser::Configuration;

pub fn build_plan(configuration: Configuration) -> Plan {
    let mut plan = Plan {
        year_plans: vec![],
        form_plan: vec![],
    };
    let config = &configuration;

    let mut empty_scores = serde_json::json!({});

    // Create forms globally (not tied to specific years)
    for form in config.forms.iter() {
        empty_scores[form.id.clone()] = 0.into();
        plan.form_plan.push(FormPlan {
            id: form.id.clone(),
            name: form.name.clone(),
        });
    }
    let empty_scores = empty_scores.to_string();

    for year in config.years.iter() {
        let year_id = year.id.clone();
        let year_name = year.name.clone();

        let mut year_plan = YearPlan {
            id: year_id.clone(),
            name: year_name,
            events: vec![],
        };

        for event in config.events.iter() {
            if !configuration.is_event_applicable_to_year(event, &year.clone().id) {
                continue;
            }
            let mut applies_to_at_least_one_form = false;
            for form in config.forms.iter() {
                if configuration.is_event_applicable_to_form(event, form.clone().id) {
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
                        scores: empty_scores.clone(),
                    })
                }
            }
        }
        plan.year_plans.push(year_plan);
    }
    plan
}

#[derive(Debug)]
pub struct Plan {
    pub year_plans: Vec<YearPlan>,
    pub form_plan: Vec<FormPlan>,
}

#[derive(Debug, Clone)]

pub struct YearPlan {
    pub id: String,
    pub name: String,
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
    pub scores: String,
}
