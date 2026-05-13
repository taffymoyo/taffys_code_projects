use crate::opponents;
use crate::characters::{Character};

pub fn performance(player1: &Character, player2: &Character) -> bool {
    let p1_talent = player1.talent + opponents::random_talent();
    let p2_talent = player2.talent + opponents::random_talent();

    println!("{} and {} put on an amazing performance for the crowd", player1.name, player2.name);
    p1_talent > p2_talent
}