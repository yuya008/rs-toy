use crate::db::DB;
use actix_web::web::Data;
use actix_web::{get, HttpRequest, HttpResponse, Result};

#[get("/welcome")]
pub async fn welcome(db: Data<DB>, req: HttpRequest) -> HttpResponse {
    info!("welcome");
    let conn = db.get().expect("couldn't get db connection from pool");

    HttpResponse::Ok().finish()
}
