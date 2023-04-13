use chrono::{prelude::*};

// 获取当前时间ISO8601格式
pub fn get_current_time() -> String {
    let now = Local::now();
    now.format("%Y-%m-%dT%H:%M:%S").to_string()
}