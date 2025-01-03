use rocket::{get, post, options, serde::json::Json, http::Status};
use crate::entity::{Comment, ContactType};
use crate::mock::get_random_comment;
use serde::Deserialize;
use chrono::Utc;

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub comment: String,
    pub contact: Option<String>,
    pub contact_type: ContactType,
    pub duration: u64,  // 存活时间（秒）
}

// ================================ 创建评论 ================================

#[options("/comments")]
pub fn options_create_comment() -> &'static str {
    ""
}

#[post("/comments", format = "json", data = "<request>")]
pub async fn create_comment(request: Json<CreateCommentRequest>) -> Result<Json<Comment>, Status> {
    // 验证评论内容
    if request.comment.trim().is_empty() {
        return Err(Status::BadRequest);
    }

    // 验证联系方式
    if request.contact_type != ContactType::Anonymous && request.contact.as_ref().map_or(true, |c| c.trim().is_empty()) {
        return Err(Status::BadRequest);
    }

    // 验证持续时间
    if request.duration < 3600 || request.duration > 72 * 3600 {
        return Err(Status::BadRequest);
    }

    let mut conn = crate::redis::get_connection().map_err(|e| {
        eprintln!("Redis connection error: {}", e);
        Status::InternalServerError
    })?;
    
    // 生成新的评论 ID
    let id: u64 = redis::cmd("INCR")
        .arg("comment_id_counter")
        .query(&mut conn)
        .map_err(|e| {
            eprintln!("Redis INCR error: {}", e);
            Status::InternalServerError
        })?;

    let now = Utc::now().timestamp() as u64;
    let comment = Comment {
        id,
        contact: request.contact.clone(),
        contact_type: request.contact_type.clone(),
        comment: request.comment.clone(),
        created_at: now,
        expires_at: now + request.duration,
        is_deleted: false,
    };

    // 将评论存入 Redis
    let json = serde_json::to_string(&comment).map_err(|e| {
        eprintln!("JSON serialization error: {}", e);
        Status::InternalServerError
    })?;

    let _: () = redis::cmd("SETEX")
        .arg(format!("comment:{}", comment.id))
        .arg(request.duration)
        .arg(json)
        .query(&mut conn)
        .map_err(|e| {
            eprintln!("Redis SETEX error: {}", e);
            Status::InternalServerError
        })?;

    Ok(Json(comment))
}

// ================================ 创建评论 ================================


// ================================ 获取指定数量的随机评论 ================================

#[options("/comments/<_num>/explore")]
pub fn options_explore(_num: u64) -> &'static str {
    ""
}

/// 获取指定数量的随机评论
#[get("/comments/<num>/explore")]
pub fn explore(num: u64) -> Result<Json<Vec<Comment>>, rocket::http::Status> {
    let mut res_vec = Vec::new();

    for _ in 0..num {
        match get_random_comment() {
            Ok(Some(comment)) => res_vec.push(comment),
            Ok(None) => return Err(rocket::http::Status::NotFound),
            Err(_) => return Err(rocket::http::Status::InternalServerError),
        }
    }
    Ok(Json(res_vec))
}

// ================================ 获取指定数量的随机评论 ================================