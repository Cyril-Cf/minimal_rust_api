use crate::services::permission_service;
use crate::AppState;
use actix_web::{delete, get, post, web, HttpResponse, Responder};

#[delete("/permissions/{permission_id}")]
async fn delete_permission(
    app_state: web::Data<AppState>,
    permission_id: web::Path<String>,
) -> impl Responder {
    match permission_id.parse::<i32>() {
        Ok(permission_id) => match permission_service::delete(permission_id, &app_state.conn).await
        {
            Ok(delete_result) => match delete_result {
                Some(result) => {
                    HttpResponse::Ok().body(format!("{} row deleted", result.rows_affected))
                }
                None => HttpResponse::InternalServerError().body(
                    "No permission deleted, it's related to a user. Remove the association first!",
                ),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[post("/permissions/{new_name}")]
async fn add_permission(app_state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let new_name = path.into_inner();
    match permission_service::create(new_name.to_string(), &app_state.conn).await {
        Ok(permission) => HttpResponse::Ok().json(permission),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/permissions/{permission_id}")]
async fn find_one_permission(
    app_state: web::Data<AppState>,
    permission_id: web::Path<String>,
) -> impl Responder {
    match permission_id.parse::<i32>() {
        Ok(permission_id) => {
            match permission_service::find_one(permission_id, &app_state.conn).await {
                Ok(permission_option) => match permission_option {
                    Some(permission) => HttpResponse::Ok().json(permission),
                    None => HttpResponse::NotFound().body("permission not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[get("/permissions")]
async fn find_all_permissions(app_state: web::Data<AppState>) -> impl Responder {
    match permission_service::find_all(&app_state.conn).await {
        Ok(permissions) => HttpResponse::Ok().json(permissions),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
