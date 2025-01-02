mod config;
mod entity;
mod redis;
mod routes;
mod trigger;

mod app;

pub use redis::{init_redis, get_connection};

pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化 Redis 连接
    redis::init_redis()?;
    // 测试连接
    redis::check_connection()?;
    
    Ok(())
}
