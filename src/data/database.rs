use chrono::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::result::Error;
use dotenv::dotenv;

use crate::models::schema::visits::dsl::*;
use crate::models::visits::Visit;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn create_visit(
        &self,
        visit_ip_address: Option<String>,
        visit_page_visited: Option<String>,
    ) -> Result<Visit, Error> {
        let conn = &mut self.pool.get().unwrap();
        let new_visit = Visit {
            id: uuid::Uuid::new_v4(),
            ip_address: visit_ip_address,
            page_visited: visit_page_visited,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        diesel::insert_into(visits)
            .values(&new_visit)
            .execute(conn)
            .expect("Error saving new visit");

        Ok(new_visit)
    }
}
