use chrono::{prelude::*};

// 获取当前时间ISO8601格式
pub fn get_current_time() -> String {
    let now = Local::now();
    now.format("%Y-%m-%dT%H:%M:%S").to_string()
}

// 生成文件名
pub fn generate_file_name(dir: &str, env: &str) -> String {
    let mut dump_dir = dir;
    if dir == "" {
        dump_dir = "./dump";
    }

    create_dir_if_not_exists(dump_dir).unwrap();
    format!("{}/dump_{}_{}.json", dump_dir, env, get_current_time())
}

// 如果文件夹不存在，就创建
pub fn create_dir_if_not_exists(dir: &str) -> std::io::Result<()> {
    if !std::path::Path::new(dir).exists() {
        std::fs::create_dir(dir)?;
    }
    Ok(())
}