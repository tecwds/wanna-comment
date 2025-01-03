use redis::Commands;
use crate::entity::{Comment, ContactType};

#[cfg(debug_assertions)]
pub fn init_mock_data() -> anyhow::Result<()> {
    let mut conn = crate::redis::get_connection()?;
    
    // 模拟数据列表，包含评论内容和联系方式
    let mock_comments = vec![
        ("今天的天气真好，希望每个人都能感受到阳光的温暖。", "wx123", ContactType::Wechat),
        ("刚刚在街角的咖啡店遇到一只可爱的橘猫。", "13800138000", ContactType::Phone),
        ("终于完成了一个困扰我很久的项目。", "example@mail.com", ContactType::Email),
        ("有时候一个微笑就能让陌生人的一天变得更美好。", "wx456", ContactType::Wechat),
        ("深夜的城市很安静，能听到自己内心的声音。", "13900139000", ContactType::Phone),
    ];

    // 将模拟数据存入 Redis
    for (idx, (content, contact, contact_type)) in mock_comments.into_iter().enumerate() {
        let comment = Comment {
            id: idx as u64 + 1,
            contact: Some(contact.to_string()),
            contact_type,
            comment: content.to_string(),
            created_at: chrono::Utc::now().timestamp() as u64,
            expires_at: (chrono::Utc::now().timestamp() + 24 * 60 * 60) as u64, // 24小时后过期
            is_deleted: false,
        };
        
        let json = serde_json::to_string(&comment)?;
        
        // 使用 comment id 作为 key
        let _: () = conn.set_ex(
            format!("comment:{}", comment.id),
            json,
            24 * 60 * 60, // 24小时过期
        )?;
    }

    println!("模拟数据初始化成功！");
    Ok(())
}

// 获取随机评论
pub fn get_random_comment() -> Result<Option<Comment>, redis::RedisError> {
    let mut conn = crate::redis::get_connection()?;
    
    // 获取所有评论的 key
    let keys: Vec<String> = conn.keys("comment:*")?;
    
    if keys.is_empty() {
        return Ok(None);
    }

    // 随机选择一个 key
    let random_key = &keys[rand::random::<usize>() % keys.len()];
    
    // 获取评论内容
    let json: String = conn.get(random_key)?;
    let comment: Comment = serde_json::from_str(&json)
        .map_err(|e| redis::RedisError::from(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            e.to_string()
        )))?;

    Ok(Some(comment))
} 