use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub age: u32,
    pub gender: String,
}
