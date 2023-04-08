use crate::data::database::DBConection;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid;

use super::schema;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = schema::waitlists)]
pub struct Waitlist {
    pub id: uuid::Uuid,
    pub ip_address: Option<String>,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct CreateWaitlistDTO {
    pub email: String,
}

pub fn create_waitlist(
    conn: &mut DBConection,
    request_ip_address: Option<String>,
    user_email: String,
) -> Result<Waitlist, diesel::result::Error> {
    use crate::models::schema::waitlists::dsl::waitlists;

    let new_waitlist = Waitlist {
        id: uuid::Uuid::new_v4(),
        ip_address: request_ip_address,
        email: user_email,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    let result = diesel::insert_into(waitlists)
        .values(&new_waitlist)
        .get_result(conn);

    match result {
        Ok(waitlist) => Ok(waitlist),
        Err(_) => Err(diesel::result::Error::NotFound),
    }
}

pub fn get_waitlist(
    conn: &mut DBConection,
    waitlist_id: uuid::Uuid,
) -> Result<Waitlist, diesel::result::Error> {
    use crate::models::schema::waitlists::dsl::waitlists;

    let result = waitlists
        .filter(schema::waitlists::id.eq(waitlist_id))
        .first::<Waitlist>(conn);

    match result {
        Ok(waitlist) => Ok(waitlist),
        Err(_) => Err(diesel::result::Error::NotFound),
    }
}

pub fn delete_waitlist(
    conn: &mut DBConection,
    waitlist_id: uuid::Uuid,
) -> Result<usize, diesel::result::Error> {
    use crate::models::schema::waitlists::dsl::waitlists;

    let result =
        diesel::delete(waitlists.filter(schema::waitlists::id.eq(waitlist_id))).execute(conn);

    match result {
        Ok(waitlist) => Ok(waitlist),
        Err(_) => Err(diesel::result::Error::NotFound),
    }
}
