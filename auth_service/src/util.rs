pub fn login(cred: models::Credentials) -> Result<database::Status, String> {
    if cred.username == "admin" && cred.password == "password" {
        Ok(crate::database::Status::Connected)
    } else {
        Err("Invalid credentials".into())
    }
}

pub mod models {
    pub struct Credentials {
        pub username: String,
        pub password: String,
    }
}
