use crate::db::{self, DBError, DB};
use crate::logger::{self, LoggerError};
use crate::opt::{Opt, OptError};
use actix_web::{middleware, rt, web, App, HttpResponse, HttpServer};
use std::io;
use thiserror::Error;

pub fn run(opt: &Opt) -> Result<(), ServerError> {
    opt.check()?;

    logger::setup_logger()?;
    let db_pool = db::new(opt)?;

    Ok(rt::System::new("rs-toy").block_on(
        HttpServer::new(move || {
            App::new()
                .data(db_pool.clone())
                .wrap(middleware::Logger::default())
                .default_service(web::to(|| HttpResponse::NotFound()))
                .configure(register_app_service)
        })
        .bind(&opt.addr)?
        .run(),
    )?)
}

fn register_app_service(cfg: &mut web::ServiceConfig) {
    use crate::handler::test;
    cfg.service(test::welcome);
}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("server configuration error: {0}")]
    Opt(#[from] OptError),

    #[error("server io error: {0}")]
    IO(#[from] io::Error),

    #[error("server db error: {0}")]
    DB(#[from] DBError),

    #[error("server logger error: {0}")]
    Logger(#[from] LoggerError),
}
