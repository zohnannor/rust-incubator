use std::path::PathBuf;

use clap::Parser;

mod settings;

#[derive(clap::Parser)]
#[clap(author, version, long_about = None)]
struct Args {
    /// Enables debug mode
    #[clap(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Path to configuration file
    #[clap(short, long, env = "CONF_FILE", default_value = "config.toml")]
    conf: PathBuf,
}

fn main() {
    let Args { debug, conf } = Args::parse();

    let config = config::Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap();

    dbg!(debug, conf);
}
