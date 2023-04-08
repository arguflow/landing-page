use actix_web::{test, App};
use llm_docs_rs::services::survey_service::scrape;

#[actix_web::test]
#[ignore = "Not yet implemented"]
async fn build_docs() {
    let mut app = test::init_service(App::new().service(scrape)).await;

    // Test 404 for empty tail
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 404);
}
