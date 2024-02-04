use crate::entities::user::{Password, User, UserId, UserName, UserRoles};
use async_trait::async_trait;
use sqlx::error::BoxDynError;
use sqlx::{query, Transaction};
use std::cmp::{PartialEq, PartialOrd};
use std::format;
use uuid::Uuid;

pub async fn find_user_by_id(id: &str) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
    let user = query!("SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&mut conn)
        .await?;

    Err("Not implemented".into())
}
