#![allow(unused)]

use chrono::{Datelike, NaiveDate};

const NOW: &str = "2019-06-26";

struct User(NaiveDate);

impl User {
    fn with_birthdate(year: i32, month: u32, day: u32) -> Self {
        Self(NaiveDate::from_ymd(year, month, day))
    }

    /// Returns current age of [`User`] in years.
    fn age(&self) -> u16 {
        let now = NaiveDate::parse_from_str(NOW, "%Y-%m-%d").unwrap();

        #[allow(
            clippy::cast_possible_wrap,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss
        )]
        {
            let now_year = now.year();
            let user_year = self.0.year();
            let now_month = now.month();
            let user_month = self.0.month();
            let now_day = now.day();
            let user_day = self.0.day();

            dbg!(now_year, user_year, now_month, user_month, now_day, user_day);

            let age = if user_month <= now_month && user_day <= now_day {
                now_year - user_year
            } else {
                now_year - user_year - 1
            };
            dbg!(dbg!(age).try_into().unwrap_or(0))
        }
    }

    /// Checks if [`User`] is 18 years old at the moment.
    fn is_adult(&self) -> bool {
        self.age() >= 18
    }
}

#[cfg(test)]
mod age_spec {
    use super::*;

    #[test]
    fn counts_age() {
        for ((y, m, d), expected) in [
            ((1990, 6, 4), 29),
            ((1990, 7, 4), 28),
            ((0, 1, 1), 2019),
            ((1970, 1, 1), 49),
            ((2019, 6, 25), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn zero_if_birthdate_in_future() {
        for ((y, m, d), expected) in [
            ((2032, 6, 25), 0),
            ((2019, 6, 27), 0),
            ((3000, 6, 27), 0),
            ((9999, 6, 27), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn adult() {
        for ((y, m, d), expected) in [
            ((1990, 6, 4), true),
            ((1990, 7, 4), true),
            ((0, 1, 1), true),
            ((1970, 1, 1), true),
            ((2019, 6, 25), false),
            ((2001, 6, 24), true),
            ((2001, 6, 25), true),
            ((2001, 6, 26), true),
            ((2001, 6, 27), false),
            ((2032, 6, 25), false),
            ((2019, 6, 27), false),
            ((3000, 6, 27), false),
            ((9999, 6, 27), false),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.is_adult(), expected, "{:?}", (y, m, d));
        }
    }
}
