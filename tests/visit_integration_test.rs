use actix_web::{test, App, middleware::Logger, web};
use llm_landing_page::models::visits::{CreateVisitDTO, Visit};
use llm_landing_page::services::visit_service::{create_visit, delete_visit, get_visit};
use diesel::{prelude::*, r2d2};

#[actix_web::test]
async fn visit_integration_test() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");


    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(create_visit)
            .service(get_visit)
            .service(delete_visit)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    )
    .await;

    // test 404 for non-existent visit
    let req = test::TestRequest::get()
        .uri("/visits/00000000-0000-0000-0000-000000000000")
        .to_request();
    let http_resp = test::call_service(&mut app, req).await;
    assert_eq!(http_resp.status(), 404);

    // test create visit
    let create_visit_dto = CreateVisitDTO {
        page_visited: Some("test".to_string()),
    };
    let req = test::TestRequest::post()
        .uri("/visits")
        .set_json(&create_visit_dto)
        .to_request();
    let resp_body: Visit = test::call_and_read_body_json(&mut app, req).await;
    assert_eq!(resp_body.page_visited, Some("test".to_string()));

    // test get visit
    let uuid = resp_body.id.to_string();
    let req = test::TestRequest::get()
        .uri(&format!("/visits/{}", uuid))
        .to_request();
    let resp_body: Visit = test::call_and_read_body_json(&mut app, req).await;
    assert_eq!(resp_body.page_visited, Some("test".to_string()));
    let req = test::TestRequest::get()
        .uri(&format!("/visits/{}", uuid))
        .to_request();
    let http_resp = test::call_service(&mut app, req).await;
    assert_eq!(http_resp.status(), 200);

    // test delete visit
    let req = test::TestRequest::delete()
        .uri(&format!("/visits/{}", uuid))
        .to_request();
    let http_resp = test::call_service(&mut app, req).await;
    assert_eq!(http_resp.status(), 200);
}