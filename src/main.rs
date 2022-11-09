use actix_web::{web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

mod controllers;
mod services;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
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
                    .service(controllers::user_controller::add_user)
                    .service(controllers::user_controller::find_one_user)
                    .service(controllers::user_controller::find_all_users)
                    .service(controllers::user_controller::update_user)
                    .service(controllers::user_controller::delete_user),
            )
            .app_data(web::Data::new(state.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
