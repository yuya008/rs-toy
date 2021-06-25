use crate::db::DB;
use actix_web::web::Data;
use actix_web::{get, HttpRequest, HttpResponse, Result};
use diesel::Connection;

#[get("/welcome")]
pub async fn welcome(db: Data<DB>, req: HttpRequest) -> HttpResponse {
    info!("welcome");
    let conn = db.get().expect("couldn't get db connection from pool");
    match conn.execute("select * from test") {
        Err(err) => {
            warn!("welcome {}", err);
            HttpResponse::BadRequest().finish()
        }
        Ok(n) => {
            info!("welcome {}", n);
            HttpResponse::Ok().finish()
        }
    }
}
