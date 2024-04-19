// Normal struct which owns the String types.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like struct.
struct AlwaysEqual;

fn main() {
    let mut user1 = build_user(
        String::from("someusername1"),
        String::from("someone@example.com")
    );

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        username: String::from("someusername2"),
        ..user1
    };

    println!("User1 username is: {}", user1.username);
    // println!("User1 email is: {}", user1.email); // Compiling error as user1.email has been moved to user2.email.

    // Tuple structs.
    let white = Color(255, 255, 255);
    // let origin = Point(0, 0, 0);

    print_color(&white);
    // print_color(&origin); // Even struct Color and Point contain exact the same fields, they're not same types.

    // Unit-like struct.
    // let unit_like_struct = AlwaysEqual;
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn print_color(color: &Color) {
    println!("Color is: {} {} {}", color.0, color.1, color.2);
}
