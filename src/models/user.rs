use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Serialize,Deserialize, FromRow,Debug)]
pub struct User {
    id: Uuid,
    name: String,
    passwd: String,
}
