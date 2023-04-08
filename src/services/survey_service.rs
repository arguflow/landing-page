use actix_web::{get, HttpRequest, HttpResponse};

#[get("/{tail:.*}")]
pub async fn scrape(req: HttpRequest) -> HttpResponse {
    let tail = req.match_info().get("tail").unwrap_or("");
    if tail.is_empty() || tail.contains("releases") || tail == "/" {
        return HttpResponse::NotFound().finish();
    }

    let crate_name = tail.split('/').next().unwrap_or("").to_owned();
    if crate_name.is_empty() {
        return HttpResponse::NotFound().finish();
    }

    HttpResponse::NoContent().finish()
}
