use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, async_graphql::SimpleObject)]
pub struct User {
    pub id: Uuid,
    pub email: String,
}
