use env_logger::Builder;
use thiserror::Error;

pub fn setup_logger() -> Result<(), LoggerError> {
    Builder::from_env("TOY_LOG").init();
    Ok(())
}

#[derive(Error, Debug)]
pub enum LoggerError {}
