use crate::data::database::DBConection;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid;

use super::schema::{self};

#[derive(
    Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset, Identifiable,
)]
#[diesel(table_name = schema::surveys)]
pub struct Survey {
    pub id: uuid::Uuid,
    pub question: String,
    pub answer: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct CreateSurveyDTO {
    pub question: String,
    pub answer: String,
}

#[derive(Serialize, Deserialize)]
pub struct SurveyPercentageDTO {
    pub answer: String,
    pub percentage: f64,
}

#[derive(Serialize, Deserialize)]
pub struct GetSurveyPercentagesDTO {
    pub question: String,
}

pub fn create_survey(
    conn: &mut DBConection,
    create_survey_dto: CreateSurveyDTO,
) -> Result<Survey, diesel::result::Error> {
    use crate::models::schema::surveys::dsl::surveys;

    let new_survey = Survey {
        id: uuid::Uuid::new_v4(),
        question: create_survey_dto.question,
        answer: create_survey_dto.answer,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    let result = diesel::insert_into(surveys)
        .values(&new_survey)
        .get_result(conn);

    match result {
        Ok(survey) => Ok(survey),
        Err(_) => Err(diesel::result::Error::NotFound),
    }
}

pub fn get_survey(
    conn: &mut DBConection,
    survey_id: uuid::Uuid,
) -> Result<Survey, diesel::result::Error> {
    use crate::models::schema::surveys::dsl::surveys;

    let result = surveys
        .filter(schema::surveys::id.eq(survey_id))
        .first::<Survey>(conn);

    match result {
        Ok(survey) => Ok(survey),
        Err(_) => Err(diesel::result::Error::NotFound),
    }
}

pub fn delete_survey(
    conn: &mut DBConection,
    survey_id: uuid::Uuid,
) -> Result<usize, diesel::result::Error> {
    use crate::models::schema::surveys::dsl::surveys;

    let result = diesel::delete(surveys.filter(schema::surveys::id.eq(survey_id))).execute(conn);

    match result {
        Ok(count) => Ok(count),
        Err(_) => Err(diesel::result::Error::NotFound),
    }
}

pub fn get_survey_percentages(
    conn: &mut DBConection,
    get_survey_percentages_dto: GetSurveyPercentagesDTO,
) -> Result<Vec<SurveyPercentageDTO>, diesel::result::Error> {
    use crate::models::schema::surveys::dsl::surveys;

    let result = surveys
        .filter(schema::surveys::question.eq(get_survey_percentages_dto.question))
        .load::<Survey>(conn);

    match result {
        Ok(vec_surveys) => {
            let mut survey_percentages: Vec<SurveyPercentageDTO> = Vec::new();
            let total_surveys = vec_surveys.len();
            let mut survey_answers: Vec<String> = vec_surveys
                .iter()
                .map(|survey| survey.answer.clone())
                .collect();
            survey_answers.sort();
            survey_answers.dedup();
            for survey_answer in survey_answers {
                let mut survey_count = 0;
                for survey in &vec_surveys {
                    if survey.answer == survey_answer {
                        survey_count += 1;
                    }
                }
                let survey_percentage = (survey_count as f64 / total_surveys as f64) * 100.0;
                survey_percentages.push(SurveyPercentageDTO {
                    answer: survey_answer,
                    percentage: survey_percentage,
                });
            }
            Ok(survey_percentages)
        }
        Err(_) => Err(diesel::result::Error::NotFound),
    }
}
