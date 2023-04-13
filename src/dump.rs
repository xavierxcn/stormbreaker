use std::io::Write;
use serde_json::Result;
use crate::config::Config;
use crate::database::dump_from_mysql;
use crate::utils::generate_file_name;

extern crate mysql;

pub fn dump(config: &Config, env: &str) -> Result<()> {
    println!("dumping {}...", env);
    println!("config: {:?}", config);

    let dump = dump_from_mysql(config, env).unwrap();

    // 将tables以json格式写入文件
    let json = serde_json::to_string(&dump).unwrap();
    println!("json: {}", json);

    // 写入文件
    let mut file = std::fs::File::create(generate_file_name(env)).unwrap();
    file.write_all(json.as_bytes()).unwrap();

    return Ok(())
}