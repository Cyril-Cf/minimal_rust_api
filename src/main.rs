use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use entity::user::Entity as User;
use migration::{Migrator, MigratorTrait};
use sea_orm::entity::ActiveValue;
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::ModelTrait;
use sea_orm::{Database, DatabaseConnection};

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[delete("/users/{user_id}")]
async fn delete_user(app_state: web::Data<AppState>, user_id: web::Path<String>) -> impl Responder {
    let user_to_delete = User::find_by_id(user_id.parse::<i32>().unwrap())
        .one(&app_state.conn)
        .await
        .unwrap();
    match user_to_delete {
        None => HttpResponse::NotFound().body("User not found"),
        Some(user) => {
            let res = user.delete(&app_state.conn).await.unwrap();
            HttpResponse::Ok().body(format!("{} row affected", res.rows_affected))
        }
    }
}

#[put("/users/{user_id}")]
async fn update_user(
    app_state: web::Data<AppState>,
    user_id: web::Path<String>,
    user_from_request: web::Json<entity::user::UpdateModel>,
) -> impl Responder {
    let user_to_update = User::find_by_id(user_id.parse::<i32>().unwrap())
        .one(&app_state.conn)
        .await
        .unwrap();
    match user_to_update {
        None => HttpResponse::NotFound().body("User not found"),
        Some(user_in_db) => {
            if user_from_request.name != user_in_db.name {
                let mut active_model = user_in_db.into_active_model();
                active_model.name = ActiveValue::Set(user_from_request.name.to_owned());
                HttpResponse::Ok().json(active_model.update(&app_state.conn).await.unwrap())
            } else {
                HttpResponse::Ok().body("No change detected, nothing applied")
            }
        }
    }
}

#[post("/users/{new_name}")]
async fn add_user(app_state: web::Data<AppState>, new_name: web::Path<String>) -> impl Responder {
    let new_user = entity::user::ActiveModel {
        name: ActiveValue::Set(new_name.to_string()),
        ..Default::default()
    };
    let inserted_user = new_user.insert(&app_state.conn).await.unwrap();
    HttpResponse::Ok().json(inserted_user)
}

#[get("/users/{user_id}")]
async fn find_one_user(
    app_state: web::Data<AppState>,
    user_id: web::Path<String>,
) -> impl Responder {
    let user = User::find_by_id(user_id.parse::<i32>().unwrap())
        .one(&app_state.conn)
        .await
        .unwrap();
    match user {
        None => HttpResponse::NotFound().body("User not found"),
        Some(user) => HttpResponse::Ok().json(user),
    }
}

#[get("/users")]
async fn find_all_users(app_state: web::Data<AppState>) -> impl Responder {
    let res = User::find().all(&app_state.conn).await.unwrap();
    HttpResponse::Ok().json(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = "sqlite::memory:";
    let conn = Database::connect(db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();
    let state = AppState { conn };

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("api")
                    .service(add_user)
                    .service(find_one_user)
                    .service(find_all_users)
                    .service(update_user)
                    .service(delete_user),
            )
            .app_data(web::Data::new(state.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
