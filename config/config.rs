
use serde;
use std;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    driver: String,
    envs: Vec<EnvConfig>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EnvConfig {
    user: String,
    password: String,
    host: String,
    port: u16,
    database: String,
    static_tables: Vec<String>,
}

impl Config {
    pub fn from_file(file_path: &str) -> std::io::Result<Self> {
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        let config = serde_yaml::from_reader(reader).unwrap();

        Ok(config)
    }
}