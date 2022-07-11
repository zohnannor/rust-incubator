use std::sync::Mutex;

use slog::Drain;

fn drain<E, I>(stderr: E, stdout: I) -> impl Drain<Ok = (), Err = slog::Never>
where
    E: std::io::Write,
    I: std::io::Write,
{
    fn kv<W: std::io::Write>(json: slog_json::JsonBuilder<W>) -> slog_json::JsonBuilder<W> {
        json.add_key_value(slog::o!(
            "msg" => slog::PushFnValue(move |record : &slog::Record, ser| {
                ser.emit(record.msg())
            }),
            "time" => slog::FnValue(move |_ : &slog::Record| {
                    time::OffsetDateTime::now_utc()
                    .format(&time::format_description::well_known::Rfc3339)
                    .ok()
            }),
            "lvl" => slog::FnValue(move |r : &slog::Record| r.level().as_str()),
        ))
    }

    let warn = kv(slog_json::Json::new(stderr)).build();
    let info = kv(slog_json::Json::new(stdout)).build();
    Mutex::new(slog::Duplicate(
        warn.filter(|r| r.level() >= slog::Level::Warning),
        info.filter(|r| r.level() < slog::Level::Warning),
    ))
    .fuse()
}

fn main() {
    let _global = slog_scope::set_global_logger(slog::Logger::root(
        drain(std::io::stderr(), std::io::stdout()),
        slog::o!("file" => "app.log"),
    ));

    slog_scope::crit!("Critical error");
    slog_scope::error!("Error occurred");
    slog_scope::warn!("Warning");
    slog_scope::debug!("Debug");
    slog_scope::info!("Info");
    slog_scope::trace!("Trace");

    let access_log = std::fs::File::options()
        .append(true)
        .create(true)
        .open("access.log")
        .expect("cannot open file access.log");

    slog_scope::scope(
        &slog::Logger::root(
            drain(access_log.try_clone().unwrap(), access_log),
            slog::o!("file" => "access.log"),
        ),
        || {
            access("POST", "/path");
        },
    );
}

fn access(method: &str, path: &str) {
    slog_scope::info!("http"; "path" => path, "method" => method);
}
