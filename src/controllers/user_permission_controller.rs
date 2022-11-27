use crate::services::user_permission_service;
use crate::AppState;
use actix_web::{delete, get, post, web, HttpResponse, Responder};

#[post("/user_permissions/{id_permission}/{id_user}")]
async fn add_permission_to_user(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (id_permission, id_user) = path.into_inner();
    match user_permission_service::create(id_permission, id_user, &app_state.conn).await {
        Ok(user_permission) => HttpResponse::Ok().json(user_permission),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/user_permissions/{id_user}")]
async fn get_permissions_for_user(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id_user = path.into_inner();
    match user_permission_service::find_all_for_user(id_user, &app_state.conn).await {
        Ok(user_permissions) => HttpResponse::Ok().json(user_permissions),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/user_permissions/{id_permission}/{id_user}")]
async fn remove_permissions_for_user(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (id_permission, id_user) = path.into_inner();
    match user_permission_service::delete(id_user, id_permission, &app_state.conn).await {
        Ok(delete_result) => match delete_result {
            Some(result) => {
                HttpResponse::Ok().body(format!("{} row deleted", result.rows_affected))
            }
            None => HttpResponse::Ok().body("No permission deleted"),
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
