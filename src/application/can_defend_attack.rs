use rand::Rng;

use crate::domain::base_character::CharacterActions;

pub fn can_defend_attack (defender: &mut dyn CharacterActions) -> bool {
    let rand = rand::thread_rng().gen_range(1..10);
    if rand > defender.get_agility() { false}
    else {
        println!("Attack defended!");
        true
    }



}
