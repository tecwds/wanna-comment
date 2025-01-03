use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// 用户评论的内容
#[derive(Serialize, Deserialize)]
pub struct Comment {
    /// 用户 ID
    pub id: u64,

    /// 用户联系方式
    pub contact: Option<String>,

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

#[derive(Debug,Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContactType {
    /// 微信
    Wechat,

    /// 电话
    Phone,

    /// 邮箱
    Email,

    /// 匿名
    Anonymous,
}

impl Display for ContactType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
