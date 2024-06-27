use crate::models::user::User;
use crate::{db::DbPool, models::user::NewUser};
use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use log::info;

#[get("/users")]
async fn get_user(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = pool.get().unwrap();
    let all_users: Result<Vec<User>, diesel::result::Error> =
        web::block(move || User::find_all(&mut conn)).await.unwrap();

    if let Ok(all_users) = all_users {
        let cloned_users: Vec<User> = all_users.clone();
        info!("Fetched all users: {:?}", cloned_users);
        HttpResponse::Ok().json(cloned_users)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[post("/users")]
async fn create_user(pool: web::Data<DbPool>, item: web::Json<NewUser>) -> impl Responder {
    let new_user: NewUser = item.into_inner();
    let mut conn: PooledConnection<ConnectionManager<PgConnection>> = pool.get().unwrap();
    let created_user: Result<User, diesel::result::Error> =
        web::block(move || User::create(new_user, &mut conn))
            .await
            .unwrap();

    if let Ok(created_user) = created_user {
        info!("Created user: {:?}", created_user);
        HttpResponse::Ok().json(created_user)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
