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

    if verify("devil2", &user1.password).unwrap() {
        println!("Id user: {}\nUsername: {}\nStatus: {}\nSign in count: {}\nPassword: {}\n", user1.id, user1.username, user1.active, user1.sign_in_count, user1.password);
    } else {
        println!("password is wrong")
    }

    println!("Id user: {}\nUsername: {}\nStatus: {}\nSign in count: {}\nPassword: {}\n", user2.id, user2.username, user2.active, user2.sign_in_count, user2.password);

}