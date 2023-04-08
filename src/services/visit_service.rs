use actix_web::{delete, get, post, web, HttpRequest, HttpResponse};

use crate::data::database::DBPool;

#[get("/visits/{visit_id}")]
pub async fn get_visit(pool: web::Data<DBPool>, visit_id: web::Path<String>) -> HttpResponse {
    let visit_id = visit_id.into_inner();
    let visit_id = match uuid::Uuid::parse_str(&visit_id) {
        Ok(visit_id) => visit_id,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    let visit = web::block(move || {
        let mut conn = pool.get().unwrap();
        crate::models::visits::get_visit(&mut conn, visit_id)
    })
    .await
    .unwrap();

    match visit {
        Ok(visit) => HttpResponse::Ok().json(visit),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[delete("/visits/{visit_id}")]
pub async fn delete_visit(pool: web::Data<DBPool>, visit_id: web::Path<String>) -> HttpResponse {
    let visit_id = visit_id.into_inner();
    let visit_id = match uuid::Uuid::parse_str(&visit_id) {
        Ok(visit_id) => visit_id,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    let visit = web::block(move || {
        let mut conn = pool.get().unwrap();
        crate::models::visits::delete_visit(&mut conn, visit_id)
    })
    .await
    .unwrap();

    match visit {
        Ok(visit) => HttpResponse::Ok().json(visit),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("/visits")]
pub async fn create_visit(
    pool: web::Data<DBPool>,
    req: HttpRequest,
    visit: web::Json<crate::models::visits::CreateVisitDTO>,
) -> HttpResponse {
    let get_client_ip = || {
        req.connection_info()
            .realip_remote_addr()
            .map(|ip| ip.to_string())
    };
    let client_ip = get_client_ip();

    let visit = web::block(move || {
        let mut conn = pool.get().unwrap();
        crate::models::visits::create_visit(&mut conn, client_ip, visit.page_visited.clone())
    })
    .await
    .unwrap();

    match visit {
        Ok(visit) => HttpResponse::Created().json(visit),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
