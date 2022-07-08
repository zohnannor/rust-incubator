use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    r#type: String,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Debug {
    #[serde(with = "humantime_serde")]
    duration: std::time::Duration,
    at: chrono::DateTime<chrono::offset::FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gift {
    id: i64,
    price: i64,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stream {
    user_id: uuid::Uuid,
    is_private: bool,
    settings: i64,
    shard_url: String,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicTariff {
    id: i64,
    price: i64,
    #[serde(with = "humantime_serde")]
    duration: std::time::Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateTariff {
    client_price: i64,
    #[serde(with = "humantime_serde")]
    duration: std::time::Duration,
    description: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json: Request =
        serde_json::from_reader(fs::File::open("./3_ecosystem/3_6_serde/request.json")?)?;

    let yaml = serde_yaml::to_string(&json)?;
    let toml = toml::to_string_pretty(&json)?;

    eprintln!("{json:#?}");

    println!("{yaml}");
    println!("{toml}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml() {
        let json_before = fs::read_to_string("request.json").unwrap();

        let yaml = {
            let r: Request = serde_json::from_str(&json_before).unwrap();
            serde_yaml::to_string(&r).unwrap()
        };

        let json_after = {
            let r: Request = serde_yaml::from_str(&yaml).unwrap();
            serde_json::to_string_pretty(&r).unwrap()
        };

        pretty_assertions::assert_eq!(json_before, json_after);
    }

    #[test]
    fn test_toml() {
        let json_before = fs::read_to_string("request.json").unwrap();

        let toml = {
            let r: Request = serde_json::from_str(&json_before).unwrap();
            toml::to_string(&r).unwrap()
        };

        let json_after = {
            let r: Request = toml::from_str(&toml).unwrap();
            serde_json::to_string_pretty(&r).unwrap()
        };

        pretty_assertions::assert_eq!(json_before, json_after);
    }
}
