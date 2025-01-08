extern crate utils;

use crate::utils::password::{compare_password, hash_password};

fn main() {
    let password = "my_secret_password";
    let hashed_password = hash_password(password.to_string()).expect("failed to hashed password");
    println!(
        "password: {}\nhashed password: {}",
        password,
        hashed_password.clone()
    );

    let is_password_valid = compare_password(password.to_string(), hashed_password.clone());
    println!("Is password valid? {}", is_password_valid)
}
