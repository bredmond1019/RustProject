use crate::db::DbPool;
use crate::models::user::User;
use actix_web::{get, web, HttpResponse, Responder};
use log::info;
// use serde::Deserialize;

// #[post("/users")]
// async fn create_user(pool: web::Data<DbPool>, item: web::Json<NewUser>) -> impl Responder {
//     // Logic for creating a user
//     pool.get()
//     HttpResponse::Ok().json("User created")
// }

#[get("/users")]
async fn get_user(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let all_users = web::block(move || User::find_all(&mut conn)).await.unwrap();

    if let Ok(all_users) = all_users {
        let cloned_users = all_users.clone();
        info!("Fetched all users: {:?}", cloned_users);
        HttpResponse::Ok().json(cloned_users)
    } else {
        // Handle the error case
        // For example, you can log the error or return an error response
        HttpResponse::InternalServerError().finish()
    }

    // HttpResponse::Ok().json("User fetched")
}
