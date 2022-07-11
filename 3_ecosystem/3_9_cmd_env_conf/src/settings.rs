use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Config {
    mode: Mode,
    server: Server,
    db: Db,
    log: Log,
    background: Background,
}

#[derive(Deserialize, Default)]
pub struct Mode {
    debug: bool,
}

#[derive(Deserialize)]
pub struct Server {
    external_url: String,
    http_port: i64,
    grpc_port: i64,
    healthz_port: i64,
    metrics_port: i64,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            external_url: "http://127.0.0.1".to_string(),
            http_port: 8081,
            grpc_port: 8082,
            healthz_port: 10025,
            metrics_port: 9199,
        }
    }
}

#[derive(Deserialize, Default)]
pub struct Db {
    mysql: Mysql,
}

#[derive(Deserialize)]
pub struct Mysql {
    host: String,
    port: i64,
    dating: String,
    user: String,
    pass: String,
    connections: Connections,
}

impl Default for Mysql {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3306,
            dating: "default".to_string(),
            user: "root".to_string(),
            pass: String::default(),
            connections: Connections::default(),
        }
    }
}

#[derive(Deserialize)]
pub struct Connections {
    max_idle: i64,
    max_open: i64,
}

impl Default for Connections {
    fn default() -> Self {
        Self {
            max_idle: 30,
            max_open: 30,
        }
    }
}

#[derive(Deserialize, Default)]
pub struct Log {
    app: App,
}

#[derive(Deserialize, Default)]
pub enum LogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

#[derive(Deserialize, Default)]
pub struct App {
    level: LogLevel,
}

#[derive(Deserialize, Default)]
pub struct Background {
    watchdog: Watchdog,
}

#[derive(Deserialize)]
pub struct Watchdog {
    #[serde(with = "humantime_serde")]
    period: std::time::Duration,
    limit: i64,
    #[serde(with = "humantime_serde")]
    lock_timeout: std::time::Duration,
}

impl Default for Watchdog {
    fn default() -> Self {
        Self {
            period: std::time::Duration::from_secs(5),
            limit: 10,
            lock_timeout: std::time::Duration::from_secs(4),
        }
    }
}
