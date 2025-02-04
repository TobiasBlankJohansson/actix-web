use actix_web::get;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct TaskIdentifier{
    task_global_id:String,
}

#[get("/task/{task_global_id}")]
pub async fn get_task(TaskIdentifier: Path<TaskIdentifier>, body:Json<Struct>) -> Json<String>{
    return Json(TaskIdentifier.into_inner().task_global_id);
}