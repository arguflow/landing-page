use actix_web::{middleware::Logger, test, web, App};
use diesel::{prelude::*, r2d2};
use llm_landing_page::models::waitlists::{CreateWaitlistDTO, Waitlist};
use llm_landing_page::services::waitlist_service::{
    create_waitlist, delete_waitlist, get_waitlist,
};

#[actix_web::test]
async fn waitlist_integration_test() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(create_waitlist)
            .service(get_waitlist)
            .service(delete_waitlist)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i")),
    )
    .await;

    // test 404 for non-existent waitlist
    let req = test::TestRequest::get()
        .uri("/waitlists/00000000-0000-0000-0000-000000000000")
        .to_request();
    let http_resp = test::call_service(&mut app, req).await;
    assert_eq!(http_resp.status(), 404);

    // test create waitlist
    let create_waitlist_dto = CreateWaitlistDTO {
        email: "test@test.com".to_string(),
    };
    let req = test::TestRequest::post()
        .uri("/waitlists")
        .set_json(&create_waitlist_dto)
        .to_request();
    let resp_body: Waitlist = test::call_and_read_body_json(&mut app, req).await;
    assert_eq!(resp_body.email, "test@test.com".to_string());

    // test get waitlist
    let uuid = resp_body.id.to_string();
    let req = test::TestRequest::get()
        .uri(&format!("/waitlists/{}", uuid))
        .to_request();
    let resp_body: Waitlist = test::call_and_read_body_json(&mut app, req).await;
    assert_eq!(resp_body.email, "test@test.com".to_string());
    let req = test::TestRequest::get()
        .uri(&format!("/waitlists/{}", uuid))
        .to_request();
    let http_resp = test::call_service(&mut app, req).await;
    assert_eq!(http_resp.status(), 200);

    // test delete waitlist
    let req = test::TestRequest::delete()
        .uri(&format!("/waitlists/{}", uuid))
        .to_request();
    let http_resp = test::call_service(&mut app, req).await;
    assert_eq!(http_resp.status(), 200);
}
