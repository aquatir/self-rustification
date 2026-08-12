use std::fmt;

fn main() {
    let separator = "****";
    println!("{separator}");
    println!("structs");
    structs();

    println!("{separator}");
    println!("tuple structs");
    tuple_structs();

    println!("{separator}");
    println!("methods");
    methods();
}

fn methods() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
    dbg!(rect1.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(50);
    dbg!(&square);
}

fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{black:?}"); // Color(0, 0, 0)
    println!("{origin:?}"); // Point(0, 0, 0)

    let subject = AlwaysEqual;
    println!("{subject:?}");
}

struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Color")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .finish()
    }
}

fn structs() {
    let my_tuple = (3, "hello", String::from("keksimu"));
    let (num, str_clice, proper_string) = my_tuple;
    println!("My typle {num} {str_clice} {proper_string}");

    let user1 = User {
        active: true,
        username: String::from("my_user"),
        email: String::from("my_email@gmail.com"),
        sign_in_count: 1,
    };
    println!("user1: {user1}");

    // this is struct update syntax: creates a new user2 from user1
    // here we only moved email, so user1 can not be used, because there is also username
    // howevre check user3 below that moves both: we can use user2 after it
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("user2: {user2}");
    // println!("user1: {user1}"); // doesn't compile

    let user3 = User {
        email: String::from("yetanother@example.com"),
        username: String::from("user3"),
        ..user2
    };
    // can print both user2 and user3
    println!("user2: {user2}");
    println!("user3: {user3}");
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // this is field init shorthand, because email in struct and email
        // in parameter are both 'email', we can do it
        email,
        sign_in_count: 1,
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) active '{}' sing_in_count '{}'",
            self.username, self.email, self.active, self.sign_in_count
        )
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        return Self {
            width: size,
            height: size,
        };
    }

    // &self is a short form for self: &Self, where Self is a type in impl,
    // so self: &Rectangle is also the same

    // fn area(self: &Rectangle) -> u32 {
    // fn area(self: &Self) -> u32 {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width >= other.width && self.height >= other.width;
    }
}
