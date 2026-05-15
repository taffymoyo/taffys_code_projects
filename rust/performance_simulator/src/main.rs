mod characters;
mod opponents;
mod game;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Welcome to the great show!");
    let char = characters::create();

    println!("\nHere is your character!\n-----------------------\nName: {}, \nAge: {}, \nGender: {:?}, \nRole: {:?}, \nTalent Level: {}\n-----------------------", char.name, char.age, char.gender, char.role, char.talent);

    let mut count: i32 = 0;

    let mut opponents = match char.role {
        characters::Role::Idol => opponents::get_idols(),
        characters::Role::Actor => opponents::get_actors(),
    };

    while opponents.len() > 0 {

        let index = rand::thread_rng().gen_range(0..opponents.len());
        let opp = opponents.remove(index);
        count += 1;

        match count {
            1 => println!("Your first opponent is {}!", opp.name),
            2 => println!("Your second opponent is {}!", opp.name),
            3 => println!("Your third opponent is {}!", opp.name),
            4 => println!("Your fourth opponent is {}!", opp.name),
            5 => println!("Your fifth opponent is {}!", opp.name),
            _ => println!("Your next opponent is {}!", opp.name),
        }


        if game::performance(&char, &opp) {
            println!("Congratulations, the crowd loved your performance! {} is the winner\n", char.name);
            sleep(Duration::from_secs(1));
        } else {
            println!("Unfortunately, the crowd perfered your opponent");
            std::process::exit(0);
        }
    };

    println!("\nCongratulations, you are the best {:?}", char.role);

}