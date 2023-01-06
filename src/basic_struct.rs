use uuid::Uuid;

extern crate bcrypt;
use bcrypt::{DEFAULT_COST, hash, verify};

struct User {
    id: Uuid,
    active: bool,
    username: String,
    password: String,
    sign_in_count: u32,
}

struct Color (u32, u32, u32);
struct Point (u32, u32, u32);

pub fn _basic_struct() {
    let user1 = User {
        id: Uuid::new_v4(),
        username: String::from("admin@email.com"),
        password: hash("devil", DEFAULT_COST).unwrap(),
        active: true,
        sign_in_count: 16,
    };

    let user2 = User {
        id: Uuid::new_v4(),
        username: String::from("kitchenbatara@gmail.com"),
        password: hash("kanyehitler", DEFAULT_COST).unwrap(),
        active: true,
        sign_in_count: 100,
    };

    let _color_black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    if verify("devil2", &user1.password).unwrap() {
        println!("Id user: {}\nUsername: {}\nStatus: {}\nSign in count: {}\nPassword: {}\n", user1.id, user1.username, user1.active, user1.sign_in_count, user1.password);
    } else {
        println!("password is wrong")
    }

    println!("Id user: {}\nUsername: {}\nStatus: {}\nSign in count: {}\nPassword: {}\n", user2.id, user2.username, user2.active, user2.sign_in_count, user2.password);
}


fn _circle_basic(width: u32, height: u32) -> u32 {
    return width * height;
}

pub fn _print_cicle_info_basic() {
    let width: u32 = 10;
    let height: u32 = 10;

    let circle = _circle_basic(width, height);
    println!("Circle width {} and hegiht {} is {}", width, height, circle);
}

fn _circle_tuple(demension: (u32, u32)) -> u32 {
    return demension.0 * demension.1;
}

pub fn _print_cicle_with_tuple() {
    let demension = (10, 10);

    let circle = _circle_tuple(demension);
    println!("Circle area is {}", circle);
}

struct Circle {
    x: u32,
    y: u32
}

// refactor code ractangle to method
impl Circle {
    fn area(&self) -> u32 {
        self.x * self.y
    }
}

pub fn _print_circle_with_struct() {
//    let area_circle = Circle {
//        x: 10,
//        y: 10,
//    };

    let ractangle = Circle { x: 10, y: 10 };

    println!("ractangle x is {} and y is {} result is {}", ractangle.x, ractangle.y, ractangle.area());
}

//fn _area_circle_with_struct(ractange: &Circle) -> u32 {
//    return ractange.x * ractange.y;
//}


