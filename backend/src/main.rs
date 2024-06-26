// use actix_web::{web, App, HttpResponse, HttpServer};

// async fn index() -> HttpResponse {
//     HttpResponse::Ok().body("Hello, Actix!")
// }

// async fn greet(name: web::Path<String>) -> HttpResponse {
//     HttpResponse::Ok().body(format!("Hello, {}!", name))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(index))
//             .route("/greet/{name}", web::get().to(greet))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod config;
mod db;
mod models;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = db::init_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
