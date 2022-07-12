#![allow(dead_code)]

use clap::Parser;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    mode: Mode,
    server: Server,
    db: Db,
    log: Log,
    background: Background,
}

#[derive(Debug, Deserialize)]
pub struct Mode {
    debug: bool,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    external_url: String,
    http_port: i64,
    grpc_port: i64,
    healthz_port: i64,
    metrics_port: i64,
}

#[derive(Debug, Deserialize)]
pub struct Db {
    mysql: Mysql,
}

#[derive(Debug, Deserialize)]
pub struct Mysql {
    host: String,
    port: i64,
    dating: String,
    user: String,
    pass: String,
    connections: Connections,
}

#[derive(Debug, Deserialize)]
pub struct Connections {
    max_idle: i64,
    max_open: i64,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    app: App,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Deserialize)]
pub struct App {
    level: LogLevel,
}

#[derive(Debug, Deserialize)]
pub struct Background {
    watchdog: Watchdog,
}

#[derive(Debug, Deserialize)]
pub struct Watchdog {
    #[serde(with = "humantime_serde")]
    period: std::time::Duration,
    limit: i64,
    #[serde(with = "humantime_serde")]
    lock_timeout: std::time::Duration,
}

#[derive(clap::Parser)]
#[clap(author, version, long_about = None)]
struct Args {
    /// Enables debug mode
    #[clap(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Path to configuration file
    #[clap(short, long, env = "CONF_FILE", default_value = "config.toml")]
    conf: std::path::PathBuf,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        let Args { debug, conf } = Args::parse();

        let config = config::Config::builder()
            .add_source(config::File::with_name(
                "3_ecosystem/3_9_cmd_env_conf/default.toml",
            ))
            .add_source(
                config::File::with_name(conf.as_os_str().to_str().expect("invalid path"))
                    .required(false),
            )
            .add_source(config::Environment::with_prefix("conf").separator("_"))
            .set_override("mode.debug", debug)?
            .build()?;

        config.try_deserialize()
    }
}
