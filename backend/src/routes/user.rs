use crate::db::DbPool;
use crate::models::user::{NewUser, User};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[post("/users")]
async fn create_user(pool: web::Data<DbPool>, item: web::Json<NewUser>) -> impl Responder {
    // Logic for creating a user
    HttpResponse::Ok().json("User created")
}

#[get("/users/{id}")]
async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> impl Responder {
    // Logic for fetching a user
    HttpResponse::Ok().json("User fetched")
}
