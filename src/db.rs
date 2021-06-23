use crate::opt::Opt;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::net::SocketAddr;
use structopt::StructOpt;
use thiserror::Error;

pub type DB = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn new(opt: &Opt) -> Result<DB, DBError> {
    info!("connect db {}", &opt.db_url);
    let manager = ConnectionManager::<MysqlConnection>::new(&opt.db_url);
    let pool = r2d2::Pool::builder().build(manager)?;
    Ok(pool)
}

#[derive(Error, Debug)]
pub enum DBError {
    #[error("create db pool: {0}")]
    Pool(#[from] r2d2::PoolError),
}
