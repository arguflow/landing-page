use actix_web::{middleware::Logger, test, web, App};
use diesel::{prelude::*, r2d2};
use llm_landing_page::models::surveys::{CreateSurveyDTO, Survey, GetSurveyPercentagesDTO, SurveyPercentageDTO};
use llm_landing_page::services::survey_service::{
    create_survey, delete_survey, get_survey, get_survey_percentages,
};

#[actix_web::test]
async fn survey_integration_test() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(create_survey)
            .service(get_survey)
            .service(delete_survey)
            .service(get_survey_percentages)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i")),
    )
    .await;

    // test 404 for non-existent survey
    let req = test::TestRequest::get()
        .uri("/surveys/00000000-0000-0000-0000-000000000000")
        .to_request();
    let http_resp = test::call_service(&mut app, req).await;
    assert_eq!(http_resp.status(), 404);

    // test create survey
    let surveys_to_create = vec![
        CreateSurveyDTO {
            question: "test".to_string(),
            answer: "answer-one".to_string(),
        },
        CreateSurveyDTO {
            question: "test".to_string(),
            answer: "answer-two".to_string(),
        },
        CreateSurveyDTO {
            question: "test".to_string(),
            answer: "answer-three".to_string(),
        },
        CreateSurveyDTO {
            question: "test".to_string(),
            answer: "answer-four".to_string(),
        },
        CreateSurveyDTO {
            question: "test".to_string(),
            answer: "answer-one".to_string(),
        },
    ];
    let mut created_surveys_uuids: Vec<String> = Vec::new();
    for survey in surveys_to_create {
        let req = test::TestRequest::post()
            .uri("/surveys")
            .set_json(&survey)
            .to_request();
        let resp_body: Survey = test::call_and_read_body_json(&mut app, req).await;
        assert_eq!(resp_body.answer, survey.answer);
        created_surveys_uuids.push(resp_body.id.to_string());
    }

    // test get survey
    for survey_uuid in created_surveys_uuids.clone() {
        let req = test::TestRequest::get()
            .uri(&format!("/surveys/{}", survey_uuid))
            .to_request();
        let resp_body: Survey = test::call_and_read_body_json(&mut app, req).await;
        assert_eq!(resp_body.id.to_string(), survey_uuid);
    }
    
    // test get survey percentages
    let req = test::TestRequest::post()
        .uri("/surveys/percentages")
        .set_json(GetSurveyPercentagesDTO {
            question: "test".to_string(),
        })
        .to_request();

    let resp_body: Vec<SurveyPercentageDTO> = test::call_and_read_body_json(&mut app, req).await;
    assert_eq!(resp_body.len(), 4);
    let answer_one_percentage = resp_body.iter().find(|x| x.answer == "answer-one").unwrap().percentage;
    let answer_two_percentage = resp_body.iter().find(|x| x.answer == "answer-two").unwrap().percentage;
    assert_eq!(answer_one_percentage, f64::from(40));
    assert_eq!(answer_two_percentage, f64::from(20));

    // test delete survey
    for survey_uuid in created_surveys_uuids.clone() {
        let req = test::TestRequest::delete()
            .uri(&format!("/surveys/{}", survey_uuid))
            .to_request();
        let http_resp = test::call_service(&mut app, req).await;
        assert_eq!(http_resp.status(), 200);
    }

}
