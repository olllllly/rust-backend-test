use rocket_contrib::json::Json;

use crate::models::User;

#[get("/")]
pub fn index() -> &'static str {
    "home"
}

#[get("/users")]
pub fn users() -> Json<Vec<User>> {
    Json(vec![User {
        id: 0,
        name: "tarou".to_string(),
        age: 20,
        gender: "female".to_string(),
    }])
}

#[post("/users", data = "<user>")]
pub fn new_user(user: Json<User>) -> String {
    format!("{:?}", user.0)
}
