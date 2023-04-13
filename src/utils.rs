use chrono::{prelude::*};

// 获取当前时间ISO8601格式
pub fn get_current_time() -> String {
    let now = Local::now();
    now.format("%Y-%m-%dT%H:%M:%S").to_string()
}

// 生成文件名
pub fn generate_file_name(env: &str) -> String {
    format!("dump_{}_{}.json", env, get_current_time())
}

