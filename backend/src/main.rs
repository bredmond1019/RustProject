use std::env;

use actix_web::web::Data;
use actix_web::{App, HttpServer};
use db::init_pool;
use dotenv::dotenv;
use models::user::{NewUser, User};

mod config;
mod db;
mod models;
mod routes;
mod schema;

fn create_test_user() {
    let pool = init_pool();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let new_user = NewUser {
        name: "Test User".to_string(),
        email: "test.user@example.com".to_string(),
    };

    match User::create(new_user, &mut conn) {
        Ok(user) => println!("Created user: {:?}", user),
        Err(e) => eprintln!("Error creating user: {:?}", e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "info"); // Set the log level
    env_logger::init(); // Initialize the logger

    let pool = db::init_pool();

    create_test_user();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
