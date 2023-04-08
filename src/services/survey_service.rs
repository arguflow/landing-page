use crate::data::database::DBPool;
use actix_web::{delete, get, post, web, HttpResponse};

#[get("/surveys/{survey_id}")]
pub async fn get_survey(pool: web::Data<DBPool>, survey_id: web::Path<String>) -> HttpResponse {
    let survey_id = survey_id.into_inner();
    let survey_id = match uuid::Uuid::parse_str(&survey_id) {
        Ok(survey_id) => survey_id,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    let survey = web::block(move || {
        let mut conn = pool.get().unwrap();
        crate::models::surveys::get_survey(&mut conn, survey_id)
    })
    .await
    .unwrap();

    match survey {
        Ok(survey) => HttpResponse::Ok().json(survey),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[delete("/surveys/{survey_id}")]
pub async fn delete_survey(pool: web::Data<DBPool>, survey_id: web::Path<String>) -> HttpResponse {
    let survey_id = survey_id.into_inner();
    let survey_id = match uuid::Uuid::parse_str(&survey_id) {
        Ok(survey_id) => survey_id,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    let survey = web::block(move || {
        let mut conn = pool.get().unwrap();
        crate::models::surveys::delete_survey(&mut conn, survey_id)
    })
    .await
    .unwrap();

    match survey {
        Ok(survey) => HttpResponse::Ok().json(survey),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("/surveys")]
pub async fn create_survey(
    pool: web::Data<DBPool>,
    create_survey_dto: web::Json<crate::models::surveys::CreateSurveyDTO>,
) -> HttpResponse {
    let survey = web::block(move || {
        let mut conn = pool.get().unwrap();
        crate::models::surveys::create_survey(&mut conn, create_survey_dto.into_inner())
    })
    .await
    .unwrap();

    match survey {
        Ok(survey) => HttpResponse::Ok().json(survey),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("/surveys/percentages")]
pub async fn get_survey_percentages(
    pool: web::Data<DBPool>,
    get_survey_percentages_dto: web::Json<crate::models::surveys::GetSurveyPercentagesDTO>,
) -> HttpResponse {
    let survey_percentages = web::block(move || {
        let mut conn = pool.get().unwrap();
        crate::models::surveys::get_survey_percentages(
            &mut conn,
            get_survey_percentages_dto.into_inner(),
        )
    })
    .await
    .unwrap();

    match survey_percentages {
        Ok(survey_percentages) => HttpResponse::Ok().json(survey_percentages),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
