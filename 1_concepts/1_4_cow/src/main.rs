use std::{borrow::Cow, env};

fn main() {
    let path: Cow<'static, str> = if let Ok(path) = env::var("APP_CONF") {
        path.into()
    } else if let Some(path) = env::args().skip_while(|a| a != "--conf").nth(1) {
        path.into()
    } else {
        "/etc/app/app.conf".into()
    };

    println!("{path}");
}
