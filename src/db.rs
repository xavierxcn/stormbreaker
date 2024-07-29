use sqlx::{MySql, Pool};
use crate::Config;

pub async fn load_db_pool(config: &Config, env: &str) -> Pool<MySql> {
    let url = format!("mysql://{}:{}@{}:{}/{}",
        config.get_env_config(env).unwrap().user,
        config.get_env_config(env).unwrap().password,
        config.get_env_config(env).unwrap().host,
        config.get_env_config(env).unwrap().port,
        config.get_env_config(env).unwrap().database,
    );

    println!("Connecting to {}...", url);

    sqlx::MySqlPool::connect(url.as_str()).await.unwrap()
}