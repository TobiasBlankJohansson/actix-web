use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Uuid;

#[derive(Deserialize, Serialize,FromRow)]
pub struct Task{
    pub id: Uuid,
    pub name: String,
}