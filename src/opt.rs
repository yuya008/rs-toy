use std::net::SocketAddr;
use structopt::StructOpt;
use thiserror::Error;

#[derive(StructOpt, Debug)]
#[structopt(name = "rs-toy")]
pub struct Opt {
    /// debug模式开关
    #[structopt(long, env = "RS_TOY_DEBUG")]
    pub debug: bool,

    /// 服务监听地址
    #[structopt(long, default_value = "0.0.0.0:8080", env = "RS_TOY_ADDR")]
    pub addr: SocketAddr,

    /// 数据库连接地址
    #[structopt(
        long,
        default_value = "mysql://root:password@127.0.0.1/test",
        env = "RS_TOY_DB_URL"
    )]
    pub db_url: String,
}

impl Opt {
    pub fn check(&self) -> Result<(), OptError> {
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum OptError {
    #[error("invalid item (item {item:?}, msg {msg:?})")]
    InvalidItem { item: String, msg: String },
}
