use crate::data::database::DBConection;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid;

use super::schema;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = schema::visits)]
pub struct Visit {
    pub id: uuid::Uuid,
    pub ip_address: Option<String>,
    pub page_visited: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct CreateVisitDTO {
    pub page_visited: Option<String>,
}

pub fn create_visit(
    conn: &mut DBConection,
    visit_ip_address: Option<String>,
    visit_page_visited: Option<String>,
) -> Result<Visit, diesel::result::Error> {
    use crate::models::schema::visits::dsl::visits;

    let new_visit = Visit {
        id: uuid::Uuid::new_v4(),
        ip_address: visit_ip_address,
        page_visited: visit_page_visited,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    let result = diesel::insert_into(visits)
        .values(&new_visit)
        .get_result(conn);

    match result {
        Ok(visit) => Ok(visit),
        Err(_) => Err(diesel::result::Error::NotFound),
    }
}

pub fn get_visit(
    conn: &mut DBConection,
    visit_id: uuid::Uuid,
) -> Result<Visit, diesel::result::Error> {
    use crate::models::schema::visits::dsl::visits;

    let result = visits
        .filter(schema::visits::id.eq(visit_id))
        .first::<Visit>(conn);

    match result {
        Ok(visit) => Ok(visit),
        Err(_) => Err(diesel::result::Error::NotFound),
    }
}

pub fn delete_visit(
    conn: &mut DBConection,
    visit_id: uuid::Uuid,
) -> Result<usize, diesel::result::Error> {
    use crate::models::schema::visits::dsl::visits;

    let result = diesel::delete(visits.filter(schema::visits::id.eq(visit_id)))
        .execute(conn);

    match result {
        Ok(visit) => Ok(visit),
        Err(_) => Err(diesel::result::Error::NotFound),
    }
}
