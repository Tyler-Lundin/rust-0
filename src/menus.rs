use std;

pub fn login_menu() {
    println!("Please enter Username and Password");
    // input username and password through command line input
    let mut username = String::new();
    let mut password = String::new();
    std::io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    std::io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");
    // trim the newline character from the end of the string
    username.pop();
    password.pop();

    // check if username and password are correct
    if username == "admin" && password == "password" {
        println!("Welcome {}", username);
        home_menu();
    } else {
        println!("Access denied");
    }
}

pub fn home_menu() {
    clr();
    println!("=== Home Menu ===");
    println!("0 - USERS");
    println!("1 - SETTINGS");
    println!("2 - EXIT");
    // input choice from user
    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    choice.pop();
    // check if choice is valid
    if choice == "0" {
        println!("Users");
        users_menu();
    } else if choice == "1" {
        println!("Settings");
        settings_menu();
    } else if choice == "2" {
        println!("Logout");
    } else {
        println!("Invalid choice!");
        home_menu();
    }
}

pub fn users_menu() {
    clr();
    println!("=== Users Menu ===");
    println!("0 - ADD USER");
    println!("1 - REMOVE USER");
    println!("2 - EDIT USER");
    println!("3 - BACK");
    // input choice from user
    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    choice.pop();
    // check if choice is valid
    if choice == "0" {
        println!("Add User");
    } else if choice == "1" {
        println!("Remove User");
    } else if choice == "2" {
        println!("Edit User");
    } else if choice == "3" {
        println!("Back");
        home_menu();
    } else {
        println!("Invalid choice!");
        users_menu();
    }
}

pub fn settings_menu() {
    clr();
    println!("=== Settings Menu ===");
    println!("0 - LANGUAGE");
    println!("1 - BACK");
    // input choice from user
    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    choice.pop();
    // check if choice is valid
    if choice == "0" {
        println!("Language");
    } else if choice == "1" {
        println!("Back");
        home_menu();
    } else {
        println!("Invalid choice!");
        settings_menu();
    }
}

fn clr() {
    clearscreen::clear().unwrap();
}
