use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}
