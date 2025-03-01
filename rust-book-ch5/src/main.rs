struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

/* structs */
fn main() {
    let mut user1 = User{
        email: String::from("mark@gmail.com"),
        username: String::from("Mark"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("Marco");

    let user2 = build_user(
        String::from("piero@gmail.com"),
        String::from("Piero"),
    );

    let user3 = User {
        email: String::from("value"),
        username: String::from("value"),
        ..user2
    };

    // a simple alternative to tuples when
    // more data structures are used
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
}

fn build_user(email: String, username:  String) -> User {
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
