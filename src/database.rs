use std::io;
use crate::config::Config;
use mysql::prelude::Queryable;
use serde::{Serialize, Deserialize};


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Table {
    pub(crate) name: String,
    pub(crate) ddl: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Database {
    pub(crate) tables: Vec<Table>,
}

// 生成mysql url
pub fn generate_mysql_url(config: &Config, env: &str) -> String {
    format!("mysql://{}:{}@{}:{}/{}",
        config.get_env_config(env).unwrap().user,
        config.get_env_config(env).unwrap().password,
        config.get_env_config(env).unwrap().host,
        config.get_env_config(env).unwrap().port,
        config.get_env_config(env).unwrap().database,
    )
}

// 从mysql dump
pub fn dump_from_mysql(config: &Config, env: &str) -> Result<Database, io::Error> {
    let url = generate_mysql_url(config, env);
    let pool = mysql::Pool::new(url.as_str()).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let mut tables: Vec<Table> = Vec::new();
    let table_names: Vec<(String,)> = conn.query("show tables").unwrap();
    for (table_name, ) in table_names {
        println!("table: {}", table_name);
        // 获取表结构DDL
        let rows: Vec<(String,String)> = conn.query(format!("show create table {}", table_name).as_str()).unwrap();

        for row in rows {
            println!("table: {:?}, ddl:{:?}\n", row.0, row.1);
            tables.push(Table {
                name: row.0,
                ddl: row.1,
            })
        }
    }

    println!("tables: {:?}", tables);
    let dump = Database{tables};
    return Ok(dump)
}