mod config;
mod entity;
mod redis;
mod routes;
mod mock;
mod cors;

pub use cors::CORS;
pub use routes::get_random;
pub use routes::options_random;
pub use routes::create_comment;
pub use routes::options_create_comment;

pub use redis::{init_redis, get_connection};
pub use mock::{init_mock_data, get_random_comment};

pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化 Redis 连接
    redis::init_redis()?;
    // 测试连接
    redis::check_connection()?;

    // 初始化模拟数据
    #[cfg(debug_assertions)]
    mock::init_mock_data()?;
    
    Ok(())
}
