use core::fmt;
use std::borrow::Borrow;

use regex::Regex;

lazy_static::lazy_static! {
    // https://stackoverflow.com/a/201378/11294165
    static ref EMAIL: Regex = Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).unwrap();
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct EmailString(String);

impl EmailString {
    pub fn new(email: impl Into<String>) -> Result<Self, InvalidEmail> {
        Self::try_from(email.into())
    }
}

impl fmt::Display for EmailString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug)]
pub struct InvalidEmail;

impl TryFrom<String> for EmailString {
    type Error = InvalidEmail;

    fn try_from(email: String) -> Result<Self, Self::Error> {
        if EMAIL.is_match(email.as_ref()) {
            Ok(Self(email))
        } else {
            Err(InvalidEmail)
        }
    }
}

impl TryFrom<&str> for EmailString {
    type Error = InvalidEmail;

    fn try_from(email: &str) -> Result<Self, Self::Error> {
        if EMAIL.is_match(email) {
            Ok(Self(email.to_string()))
        } else {
            Err(InvalidEmail)
        }
    }
}

impl AsRef<str> for EmailString {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Borrow<str> for EmailString {
    fn borrow(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    #[should_panic]
    fn feature() {
        EmailString::new("hi").unwrap();
    }

    #[test]
    fn test_email_string() {
        let email1 = EmailString::new("test@example.com").unwrap();
        let email2 = EmailString::try_from("test-1-hello@example2.com").unwrap();

        let mut map = HashMap::new();

        map.insert(email1, "user1");
        map.insert(email2, "user2");

        assert_eq!(map.get("test@example.com"), Some(&"user1"));
    }
}
