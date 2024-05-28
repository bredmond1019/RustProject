use async_graphql::{Context, Object, Schema, EmptyMutation, EmptySubscription};
use uuid::Uuid;
use crate::models::User;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users(&self, ctx: &Context<'_>) -> Vec<User> {
        let users = ctx.data::<Arc<RwLock<Vec<User>>>>().unwrap().read().await;
        users.clone()
    }
}

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema(users: Arc<RwLock<Vec<User>>>) -> AppSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(users)
        .finish()
}
