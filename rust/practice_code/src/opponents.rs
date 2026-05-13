use rand::Rng;
use crate::characters::{Character, Gender, Role};

pub fn random_talent() -> i32 {
    rand::thread_rng().gen_range(1..=10)
}

pub fn get_idols() -> Vec<Character> {
    vec![
        Character {
            name: String::from("Ruby"),
            age: 18,
            gender: Gender::Female,
            role: Role::Idol,
            talent: random_talent(),
        },
        Character {
            name: String::from("Ai"),
            age: 20,
            gender: Gender::Female,
            role: Role::Idol,
            talent: random_talent(),
        },
        Character {
            name: String::from("Mem_Cho"),
            age: 27,
            gender: Gender::Female,
            role: Role::Idol,
            talent: random_talent(),
        },
    ]
}

pub fn get_actors() -> Vec<Character> {
    vec![
        Character {
            name: String::from("Aqua"),
            age: 18,
            gender: Gender::Male,
            role: Role::Actor,
            talent: random_talent(),
        },
        Character {
            name: String::from("Kana"),
            age: 18,
            gender: Gender::Female,
            role: Role::Actor,
            talent: random_talent(),
        },
        Character {
            name: String::from("Akane"),
            age: 18,
            gender: Gender::Female,
            role: Role::Actor,
            talent: random_talent(),
        },
        Character {
            name: String::from("Melt"),
            age: 18,
            gender: Gender::Male,
            role: Role::Actor,
            talent: random_talent(),
        },
    ]
}