use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
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
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let conn = Database::connect(db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();
    println!("Backend launched!");
    let state = AppState { conn };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(
                web::scope("api")
                    .service(controllers::user_controller::add_user)
                    .service(controllers::user_controller::find_one_user)
                    .service(controllers::user_controller::find_all_users)
                    .service(controllers::user_controller::update_user)
                    .service(controllers::user_controller::delete_user)
                    .service(controllers::permission_controller::add_permission)
                    .service(controllers::permission_controller::find_all_permissions)
                    .service(controllers::permission_controller::find_one_permission)
                    .service(controllers::permission_controller::delete_permission)
                    .service(controllers::user_permission_controller::add_permission_to_user)
                    .service(controllers::user_permission_controller::get_permissions_for_user)
                    .service(controllers::user_permission_controller::remove_permissions_for_user),
            )
            .app_data(web::Data::new(state.clone()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
