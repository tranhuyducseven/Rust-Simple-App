//CRUD App

use std::io;

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("please try again...");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

enum Manager {
    AddStudent,
    ViewStudent,
    EditStudent,
    RemoveStudent,
}
impl Manager {
    fn show() {
        println!(".....");
        println!("== Manager Panel ==");
        println!("....");
        println!("1. Add student");
        println!("2. View student");
        println!("3. Edit student");
        println!("4. Remove student");
        println!("....");
        println!("Please enter your choice!");
        println!("....");
    }
    fn choice(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::AddStudent),
            "2" => Some(Manager::ViewStudent),
            "3" => Some(Manager::EditStudent),
            "4" => Some(Manager::RemoveStudent),
            _ => None,
        }
    }
}

fn main() {
    loop {
        Manager::show();
        let input = get_input().expect("Please enter your choice");
        match Manager::choice(&input.as_str()) {
            Some(Manager::AddStudent) => (),
            Some(Manager::ViewStudent) => (),
            Some(Manager::EditStudent) => (),
            Some(Manager::RemoveStudent) => (),
            None => return,
        }
    }
}
