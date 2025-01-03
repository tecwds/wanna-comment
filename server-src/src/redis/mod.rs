mod re_comment;

use once_cell::sync::OnceCell;
use redis::{Client, Connection, RedisError};
use std::sync::Mutex;

static REDIS_CLIENT: OnceCell<Mutex<Client>> = OnceCell::new();

pub fn init_redis() -> Result<(), RedisError> {
    let redis_url = "redis://:root@localhost:6379";
    let client = Client::open(redis_url)?;
    
    // 初始化全局 Redis 客户端
    REDIS_CLIENT.set(Mutex::new(client))
        .map_err(|_| RedisError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Redis client already initialized"
        )))?;
    
    Ok(())
}

pub fn get_connection() -> Result<Connection, RedisError> {
    REDIS_CLIENT
        .get()
        .ok_or_else(|| RedisError::from(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Redis client not initialized"
        )))?
        .lock()
        .map_err(|_| RedisError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Redis client lock poisoned"
        )))?
        .get_connection()
}

// 辅助函数：检查 Redis 连接是否可用
pub fn check_connection() -> Result<(), RedisError> {
    let mut conn = get_connection()?;
    redis::cmd("PING").query::<String>(&mut conn)?;
    Ok(())
}