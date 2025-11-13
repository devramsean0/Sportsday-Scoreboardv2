use crate::db_configurator::parser::DBConfiguration;

pub fn build_plan(configuration: DBConfiguration) {
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
                for event in config.events.iter() {
                    if configuration.is_event_applicable_to_form(event, form.clone().id)
                        && configuration.is_event_applicable_to_year(event, &year.clone().id)
                    {
                        for gender in config.genders.iter() {
                            if configuration.is_event_applicable_to_gender(event, gender) {
                                year_plan.events.push(EventPlan {
                                    id: format!(
                                        "{}-{}-{}",
                                        year_plan.clone().id,
                                        gender,
                                        event.clone().id
                                    ),
                                    name: event.clone().name,
                                    year_id: year_plan.id.clone(),
                                    gender_id: gender.clone(),
                                })
                            };
                        }
                    };
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
    println!("Plan: {:#?}", plan)
}

#[derive(Debug)]
struct Plan {
    year_plans: Vec<YearPlan>,
    score_plan: Vec<ScorePlan>,
}

#[derive(Debug)]
struct ScorePlan {
    name: String,
    value: i64,
}

#[derive(Debug, Clone)]

struct YearPlan {
    id: String,
    name: String,
    forms: Vec<FormPlan>,
    events: Vec<EventPlan>,
}

#[derive(Debug, Clone)]

struct FormPlan {
    id: String,
    name: String,
}

#[derive(Debug, Clone)]

struct EventPlan {
    id: String,
    name: String,
    year_id: String,
    gender_id: String,
}
