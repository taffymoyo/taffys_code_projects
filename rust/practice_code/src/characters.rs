use std::io::{self, Write};
use rand::Rng;


#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
}
#[derive(Debug)]
pub enum Role {
    Idol, 
    Actor,
}

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub age: i32,
    pub gender: Gender,
    pub role: Role,
    pub talent: i32,
}
pub fn create() -> Character { 

    println!("Make your character!");

    print!("Enter your name: ");
    io::stdout().flush().expect("Failed to flush");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to readline");
    let name = name.trim().to_string();
    
    print!("Enter your age: ");
    io::stdout().flush().expect("Failed to flush");
    
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to readline");
    let age: i32 = age.trim().parse().expect("Not a number");

    println!("Pick a gender!");

    let gender = loop {
        print!("1 - Boy, 2 - Girl: ");
        io::stdout().flush().expect("Failed to flush");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to readline");
        let choice = choice.trim();

        match choice {
            "1" => break Gender::Male,
            "2" => break Gender::Female,
            _ => println!("Please select a valid option\n"),
        };
    };

    let role = loop {
        print!("1 - Idol, 2 - Actor: ");
        io::stdout().flush().expect("Failed to flush");
        let mut role = String::new();
        io::stdin().read_line(&mut role).expect("Failed to readline");
        let role = role.trim();

        match role {
            "1" => break Role::Idol,
            "2" => break Role::Actor,
            _ => println!("Please select a valid option\n"),
        };
    };

    let talent = rand::thread_rng().gen_range(1..=10);

    println!("Finalising your character");
        Character {
            name, 
            age, 
            gender, 
            role, 
            talent
        }
}