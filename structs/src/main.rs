struct User {
    name: String,
    username: String,
    email: String,
    age: u8,
    job: bool,
}

fn main() {
    let user1 = User {
        name: String::from("Ankit"),
        username: String::from("Ankit Singh Lingwal"),
        email: String::from("ankit@example.com"),
        age: 25,
        job: true,
    };
    println!(
        "User 1: {}, {}, {}, {}, {}",
        user1.name, user1.username, user1.email, user1.age, user1.job
    );

    let user2: User = create_user(
        String::from("John"),
        String::from("John Doe"),
        String::from("john@example.com"),
        30,
    );
    println!(
        "User 2: {}, {}, {}, {}, {}",
        user2.name, user2.username, user2.email, user2.age, user2.job
    );

    let user3 = User {
        email: String::from("sas@example.com"),
        ..user2
    };
    println!(
        "User 3: {}, {}, {}, {}, {}",
        user3.name, user3.username, user3.email, user3.age, user3.job
    );
}

fn create_user(name: String, username: String, email: String, age: u8) -> User {
    User {
        name: name,
        username: username,
        email: email,
        age: age,
        job: false,
    }
}
