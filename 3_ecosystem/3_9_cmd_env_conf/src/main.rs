mod settings;

fn main() {
    let config = settings::Config::new().unwrap();

    dbg!(config);
}
