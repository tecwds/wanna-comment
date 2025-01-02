use serde::{Deserialize, Serialize};

/// 用户评论的内容
#[derive(Serialize, Deserialize)]
pub struct Comment {
    /// 用户 ID
    pub id: u64,

    /// 用户联系方式
    pub contact: String,

    /// 用户联系方式类型
    pub contact_type: ContactType,

    /// 评论内容
    pub comment: String,

    /// 创建时间
    pub created_at: u64,

    /// 过期时间
    pub expires_at: u64,

    /// 是否已删除
    pub is_deleted: bool,
}

#[derive(Serialize, Deserialize)]
pub enum ContactType {
    /// 微信
    Wechat,

    /// 电话
    Phone,

    /// 邮箱
    Email,
}
