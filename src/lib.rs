pub mod data;
pub mod models;
pub mod services;

use actix_web::{middleware::Logger, web, App, HttpServer};
use data::database::run_migrations;
use diesel::{prelude::*, r2d2};
use env_logger::Env;
use services::survey_service::{create_survey, get_survey_percentages};
use services::visit_service::create_visit;
use services::waitlist_service::create_waitlist;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    run_migrations(&mut pool.get().unwrap());

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(create_visit)
            .service(create_waitlist)
            .service(create_survey)
            .service(get_survey_percentages)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}
