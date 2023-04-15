use std::io;
use crate::config::Config;
use crate::database::{Database, dump_from_mysql};

// 比较
pub fn compare(config: &Config, src: &str, dst: &str) -> Result<(), io::Error> {
    println!("comparing {} and {}...", src, dst);
    println!("config: {:?}", config);

    let src_db = dump_from_mysql(config, src).unwrap();
    let dst_db = dump_from_mysql(config, dst).unwrap();

    // 比较
    compare_databases(&src_db, &dst_db).unwrap();

    return Ok(());
}

fn compare_databases(src_db: &Database, dst_db: &Database) -> Result<(), io::Error> {
    // 比较表结构
    for src_table in src_db.tables {
        let dst_table = dst_db.tables.iter().find(|table| table.name == src_table.name);
        if dst_table.is_none() {
            println!("table {} not found in dst", src_table.name);
            continue;
        }

        if src_table.ddl != dst_table.unwrap().ddl {
            println!("table {} ddl not match", src_table.name);
        }
    }

    // 比较表数据
    for src_table in src_db.tables {
        let dst_table = dst_db.tables.iter().find(|table| table.name == src_table.name);
        if dst_table.is_none() {
            println!("table {} not found in dst", src_table.name);
            continue;
        }

        if src_table.ddl != dst_table.unwrap().ddl {
            println!("table {} ddl not match", src_table.name);
        }
    }

    return Ok(());
}


