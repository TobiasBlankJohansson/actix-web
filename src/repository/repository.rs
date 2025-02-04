use actix_web::web;
use sqlx::PgPool;
use crate::model::task::Task;

pub async fn task_list(pool: web::Data<PgPool>) -> Vec<Task> {
    return sqlx::query_as::<_,Task>("SELECT id, name FROM task")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
}