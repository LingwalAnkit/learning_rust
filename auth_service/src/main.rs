use auth_service::auth_utils::models::Credentials;
use auth_service::connected;

fn main() {
    let cred = Credentials {
        username: "admin".to_string(),
        password: "password".to_string(),
    };

    connected(cred);
}
