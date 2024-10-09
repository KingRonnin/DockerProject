use std::env;

pub struct Config {
    pub server_address: String,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let server_address = env::var("SERVER_ADDRESS")?;
        Ok(Self { server_address })
    }
}