
use serde;
use std;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub driver: String,
    pub envs: Vec<EnvConfig>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EnvConfig {
    pub name: String,
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub static_tables: Vec<String>,
}

impl Config {
    pub fn from_file(file_path: &str) -> std::io::Result<Self> {
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        let config = serde_yaml::from_reader(reader).unwrap();

        Ok(config)
    }

    pub fn get_env_config(&self, env_name: &str) -> Option<&EnvConfig> {
        self.envs.iter().find(|env| env.name == env_name)
    }
}