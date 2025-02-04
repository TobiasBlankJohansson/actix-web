use actix_web::web;
use sqlx::PgPool;
use crate::repository::repository::task_list;

pub fn get_all_task(pool: web::Data<PgPool>) {
    return task_list(pool);
}