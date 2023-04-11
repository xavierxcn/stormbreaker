use mysql::prelude::Queryable;
use crate::config::Config;
extern crate mysql;

struct Table {
    name: String,
    ddl: String,
}

pub fn dump(config: &Config, env: &str) -> Result<(), ()> {
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

    return Ok(())
}