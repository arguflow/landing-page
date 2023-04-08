use crate::data::database::DBPool;
use actix_web::{delete, get, post, web, HttpResponse};

#[get("/waitlists/{waitlist_id}")]
pub async fn get_waitlist(pool: web::Data<DBPool>, waitlist_id: web::Path<String>) -> HttpResponse {
    let waitlist_id = waitlist_id.into_inner();
    let waitlist_id = match uuid::Uuid::parse_str(&waitlist_id) {
        Ok(waitlist_id) => waitlist_id,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    let waitlist = web::block(move || {
        let mut conn = pool.get().unwrap();
        crate::models::waitlists::get_waitlist(&mut conn, waitlist_id)
    })
    .await
    .unwrap();

    match waitlist {
        Ok(waitlist) => HttpResponse::Ok().json(waitlist),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[delete("/waitlists/{waitlist_id}")]
pub async fn delete_waitlist(
    pool: web::Data<DBPool>,
    waitlist_id: web::Path<String>,
) -> HttpResponse {
    let waitlist_id = waitlist_id.into_inner();
    let waitlist_id = match uuid::Uuid::parse_str(&waitlist_id) {
        Ok(waitlist_id) => waitlist_id,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    let waitlist = web::block(move || {
        let mut conn = pool.get().unwrap();
        crate::models::waitlists::delete_waitlist(&mut conn, waitlist_id)
    })
    .await
    .unwrap();

    match waitlist {
        Ok(waitlist) => HttpResponse::Ok().json(waitlist),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("/waitlists")]
pub async fn create_waitlist(
    pool: web::Data<DBPool>,
    create_waitlist_dto: web::Json<crate::models::waitlists::CreateWaitlistDTO>,
) -> HttpResponse {
    let waitlist = web::block(move || {
        let mut conn = pool.get().unwrap();
        crate::models::waitlists::create_waitlist(&mut conn, create_waitlist_dto.email.clone())
    })
    .await
    .unwrap();

    match waitlist {
        Ok(waitlist) => HttpResponse::Ok().json(waitlist),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
