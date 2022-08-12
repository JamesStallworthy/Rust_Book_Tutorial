#[derive(Debug)] //This allows us to output the data in the debug console
struct User {
    active: bool,
    username: String,
    password: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("james@test.com"),
        password: String::from("SuperSecret"),
        sign_in_count: 1
    };

    let username = &user1.username;
    println!("Username: {username}");

    let mut user2 = User {
        active: true,
        username: String::from("james@test.com"),
        password: String::from("SuperSecret"),
        sign_in_count: 1
    };
    
    user2.username = String::from("test@test.com");

    let username = &user2.username;

    println!("Username 2: {username}");

    //Copy syntax
    
    let user3 = User {
        username: String::from("Something@something.com"),
        ..user2
    };

    let rect = Rectangle{
        width: 5,
        height: 5
    };

    let area = rect.area();
}

fn build_user(username: String, password: String) -> User {
    User {
        username: username,
        password: password,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_shorthand(username: String, password: String) -> User {
    User {
        username,
        passwordd,
        active: true,
        sign_in_count: 1,
    }
}
