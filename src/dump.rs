use std::io::Write;
use mysql::prelude::Queryable;
use serde::{Serialize, Deserialize};
use serde_json::Result;
use crate::config::Config;
use crate::utils::get_current_time;

extern crate mysql;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Table {
    name: String,
    ddl: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Dump {
    tables: Vec<Table>,
}

pub fn dump(config: &Config, env: &str) -> Result<()> {
    println!("dumping {}...", env);
    println!("config: {:?}", config);

    let url = format!("mysql://{}:{}@{}:{}/{}",
        config.get_env_config(env).unwrap().user,
        config.get_env_config(env).unwrap().password,
        config.get_env_config(env).unwrap().host,
        config.get_env_config(env).unwrap().port,
        config.get_env_config(env).unwrap().database,
    );

    println!("connect url: {}", url);

    let mut tables: Vec<Table> = Vec::new();

    let pool = mysql::Pool::new(url.as_str()).unwrap();

    let mut conn = pool.get_conn().unwrap();
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
    let dump = Dump{tables};

    // 将tables以json格式写入文件
    let json = serde_json::to_string(&dump).unwrap();
    println!("json: {}", json);

    // 写入文件
    let filename = format!("{}-{}.json", env, get_current_time());
    let mut file = std::fs::File::create(filename).unwrap();
    file.write_all(json.as_bytes()).unwrap();

    return Ok(())
}