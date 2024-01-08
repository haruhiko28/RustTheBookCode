struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // println!("Hello, world!");
    let mut user1 = User {
        email: String::from("hoge@hoge.com"),
        username: String::from("hogehoge"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotherhoge@hoge");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}