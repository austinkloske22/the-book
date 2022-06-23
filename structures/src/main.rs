// This structure owns all of its data
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// This structure references data owned elsewhere
/*
struct UserRef {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}
*/

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    // User Scope
    {
        let static_user = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        let mut mutable_user = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        mutable_user.email = String::from("anotheremail@example.com");

        let user1 = build_user("user@gmail.com".to_owned(), "user".to_owned());

        let less_efficient_user = User {
            active: user1.active,
            username: user1.username,
            email: String::from("another@example.com"),
            sign_in_count: user1.sign_in_count,
        };

        let more_efficient_user = User {
            email: String::from("another@example.com"),
            ..less_efficient_user
        };

        /* Error -> missing lifetime specifier. This is covered later (Chapter 10)
        let user_reference = UserRef {
            email: "someone@example.com",
            username: "someusername123",
            active: true,
            sign_in_count: 1,
        };
        */
    }

    {
        let subject = AlwaysEqual;
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
        // println!("The subject is {}", subject); Error -> `AlwaysEqual` cannot be formatted with the default formatter
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
