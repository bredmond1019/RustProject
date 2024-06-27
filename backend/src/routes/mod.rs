mod home;
mod user;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(home::index);
    cfg.service(user::create_user);
    cfg.service(user::get_user);
}
