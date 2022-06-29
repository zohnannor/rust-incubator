use step_1_5::{email_string::EmailString, random::Random};

fn main() {
    let r = Random::from([
        EmailString::new("1@hi.com").unwrap(),
        EmailString::new("2@hi.com").unwrap(),
        EmailString::new("3@hi.com").unwrap(),
    ]);

    for _ in 0..10 {
        let email = &r;
        println!("{email}");
    }
}
