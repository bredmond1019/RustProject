use crate::schema::users;
use diesel::{
    deserialize::Queryable,
    prelude::Insertable,
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection, RunQueryDsl,
};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

impl User {
    pub fn find_all(
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<Vec<User>, diesel::result::Error> {
        let all_users: Result<Vec<User>, diesel::result::Error> = users::table.load(conn);
        info!("{:?}", &all_users);
        all_users
    }

    pub fn create(
        new_user: NewUser,
        conn: &mut PgConnection,
    ) -> Result<User, diesel::result::Error> {
        let user: Result<User, diesel::result::Error> = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn);
        user
    }
}
