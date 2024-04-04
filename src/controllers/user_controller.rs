use crate::services::user_service;
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[delete("/users/{user_id}")]
async fn delete_user(app_state: web::Data<AppState>, user_id: web::Path<String>) -> impl Responder {
    match user_id.parse::<i32>() {
        Ok(user_id) => match user_service::delete(user_id, &app_state.conn).await {
            Ok(delete_result) => match delete_result {
                Some(result) => {
                    HttpResponse::Ok().body(format!("{} row deleted", result.rows_affected))
                }
                None => HttpResponse::InternalServerError().body(
                    "No user deleted, it's related to a permission. Remove the association first!",
                ),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[put("/users/{user_id}")]
async fn update_user(
    app_state: web::Data<AppState>,
    user_id: web::Path<String>,
    user_from_request: web::Json<entity::user::UpdateModel>,
) -> impl Responder {
    match user_id.parse::<i32>() {
        Ok(user_id) => {
            match user_service::update(&app_state.conn, user_id, user_from_request.into_inner())
                .await
            {
                Ok(user_option) => match user_option {
                    Some(user) => HttpResponse::Ok().json(user),
                    None => HttpResponse::NotFound().body("User not found"),
                },
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[post("/users/{new_name}")]
async fn add_user(app_state: web::Data<AppState>, new_name: web::Path<String>) -> impl Responder {
    match user_service::create(new_name.to_string(), &app_state.conn).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users/{user_id}")]
async fn find_one_user(
    app_state: web::Data<AppState>,
    user_id: web::Path<String>,
) -> impl Responder {
    match user_id.parse::<i32>() {
        Ok(user_id) => match user_service::find_one(user_id, &app_state.conn).await {
            Ok(user_option) => match user_option {
                Some(user) => HttpResponse::Ok().json(user),
                None => HttpResponse::NotFound().body("User not found"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[get("/users")]
async fn find_all_users(app_state: web::Data<AppState>) -> impl Responder {
    match user_service::find_all(&app_state.conn).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
