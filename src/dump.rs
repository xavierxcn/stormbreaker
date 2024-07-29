use std::io::Write;
use serde::{Serialize, Deserialize};
use serde_json::Result;
use sqlx::mysql::MySqlPool;
use sqlx::{MySql, Pool, Row};
use crate::config::Config;
use crate::load_db_pool;
use crate::utils::generate_file_name;

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

pub async fn dump(config: &Config, env: &str) -> Result<()> {
    println!("dumping {}...", env);
    println!("config: {:?}", config);

    let pool = load_db_pool(config, env).await;
    let dump = dump_from_db(pool).await.unwrap();

    // 将tables以json格式写入文件
    let json = serde_json::to_string(&dump).unwrap();
    println!("json: {}", json);

    // 写入文件
    let mut file = std::fs::File::create(generate_file_name(env)).unwrap();
    file.write_all(json.as_bytes()).unwrap();

    return Ok(())
}

async fn dump_from_db(pool: Pool<MySql>) -> Result<Dump> {
    let mut tables: Vec<Table> = Vec::new();
    // 执行 show tables 并获取表名
    let rows = sqlx::query("SHOW TABLES;")
        .fetch_all(&pool).await.unwrap();
    for row in rows {
        let table_name: String = row.get(0);
        println!("table name: {}", table_name);
        tables.push(Table {
            name: table_name.clone(),
            ddl: get_table_ddl(&pool, table_name.clone()).await.unwrap(),
        });
    }


    println!("tables: {:?}", tables);
    let dump = Dump{tables};

    return Ok(dump)
}

async fn get_table_ddl(pool: &MySqlPool, table_name: String) -> sqlx::Result<String> {
    let ddl = sqlx::query(format!("SHOW CREATE TABLE {}", table_name).as_str())
        .fetch_one(pool)
        .await
        .unwrap();
    Ok(ddl.get(1))
}