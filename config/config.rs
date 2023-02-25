
use serde::{Deserialize, Serialize};
use std::{fs::File, io::prelude::*};

#[derive(Debug, serde::Deserialize)]
struct Config {
    driver: String,
    envs: Vec<EnvConfig>,
}

#[derive(Debug, serde::Deserialize)]
struct EnvConfig {
    user: String,
    password: String,
    host: String,
    port: u16,
    database: String,
    static_tables: Vec<String>,
}

pub fn load_from_path(path: &str) -> std::io::Result<()> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = serde_yaml::from_str(&contents)?;
    println!("{:?}", config);

    Ok(())
}
