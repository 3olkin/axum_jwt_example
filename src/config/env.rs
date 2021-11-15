use std::{env, net::IpAddr};

use clap::Parser;

lazy_static! {
    pub static ref JWT_SECRET: String = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
}

#[derive(Debug, Parser)]
pub struct ServerConfig {
    #[clap(default_value = "127.0.0.1", env)]
    pub host: IpAddr,
    #[clap(default_value = "3000", env)]
    pub port: u16,
}

#[derive(Debug, Parser)]
pub struct PgConfig {
    #[clap(required = true, env)]
    pub pg_database: String,
    #[clap(default_value = "0.0.0.0", env)]
    pub pg_host: IpAddr,
    #[clap(default_value = "5432", env)]
    pub pg_port: u16,
    #[clap(default_value = "postgres", env)]
    pub pg_user: String,
    #[clap(default_value = "", env)]
    pub pg_password: String,
}
