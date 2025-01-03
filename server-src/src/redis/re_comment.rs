//
// 对评论，Redis 相关操作
// 

use redis::{Commands, RedisError};

/// 设置评论的存续时间
/// 
/// # 参数
/// * `comment_id` - 评论ID
/// * `seconds` - 存续时间(秒)
pub fn set_comment_ttl(comment_id: &str, seconds: usize) -> Result<(), RedisError> {
    let mut conn = super::get_connection()?;
    let _: () = conn.expire(format!("comment:{}", comment_id), seconds as i64)?;
    Ok(())
}

/// 获取评论的剩余存续时间
/// 
/// # 参数
/// * `comment_id` - 评论ID
/// 
/// # 返回
/// * `Ok(Some(ttl))` - 剩余时间(秒)
/// * `Ok(None)` - 评论不存在
pub fn get_comment_ttl(comment_id: &str) -> Result<Option<usize>, RedisError> {
    let mut conn = super::get_connection()?;
    let ttl: Option<usize> = conn.ttl(format!("comment:{}", comment_id))?;
    Ok(ttl)
}

