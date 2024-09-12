use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BusinessSearch {
    pub q: String,
}

#[derive(Debug, Deserialize)]
pub struct UserRegistration {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}
