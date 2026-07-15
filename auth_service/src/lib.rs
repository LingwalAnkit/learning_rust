pub mod database {
    pub enum Status {
        Connected,
        Iterrupted,
    }

    pub fn auth_status() -> Status {
        Status::Connected
    }
}

pub mod utils;

pub fn connected(cred: auth_utils::models::Credentials) {
    if let database::Status::Connected = database::auth_status() {
        match auth_utils::login(cred) {
            Ok(_) => {
                println!("Login successful.");
            }
            Err(e) => {
                println!("Login failed: {}", e);
            }
        }
    }
}

