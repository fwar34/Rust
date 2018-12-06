fn main() {
    let mut user1 = User {
        username: String::from("Andy"),
        email: String::from("test.live.com"),
        sign_in_count: 55,
        active: true,
    };
    user1.username = String::from("Lucy");

    let _user2 = User {
        username: String::from("XXXX"),
        email: String::from("test2.live.com"),
        sign_in_count: user1.sign_in_count,
        active: user1.active,
    };

    let _user2 = User {
        username: String::from("YYYYY"),
        email: String::from("test3.live.com"),
        ..user1
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn _build_user(name: String, email: String) -> User {
    User {
        username: name,
        email: email,
        sign_in_count: 101,
        active: true,
    }
}

//字段初始化简写语法(field init shorthand)
fn _build_user2(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1000,
        active: false,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
