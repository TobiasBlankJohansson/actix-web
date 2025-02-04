use actix_web::{get, web, HttpResponse, Responder};
use actix_web::web::{Path};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::model::task::Task;

#[derive(Deserialize,Serialize)]
pub struct TaskIdentifier{
    task_global_id:String,
}

#[get("/task")]
pub async fn get_all_task() -> impl Responder{
    HttpResponse::Ok().json(tasks)
}

#[get("/task/{task_global_id}")]
pub async fn get_task(task_identifier: Path<TaskIdentifier>) -> impl Responder{
    HttpResponse::Ok().json(task_identifier.into_inner().task_global_id)
}