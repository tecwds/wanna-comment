use server_src::init;
use redis::Commands;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化应用，包括 Redis 连接
    init().await?;
    
    // 测试 Redis 连接
    let mut conn = server_src::get_connection()?;
    let test_result: Result<String, redis::RedisError> = conn.set_ex("test_startup", "ok", 60);
    match test_result {
        Ok(_) => println!("Redis 连接成功！"),
        Err(e) => eprintln!("Redis 连接失败: {}", e),
    }

    // 启动 Rocket 服务器
    let _ = rocket::build().launch().await?;
    
    Ok(())
}
