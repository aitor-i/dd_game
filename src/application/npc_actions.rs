use crate::domain::base_character::CharacterActions;
use crate::application::calculate_attack::calculate_attack;

pub fn npc_actions (npc: &mut dyn CharacterActions, chatacter: &mut dyn CharacterActions) -> () { 

    println!();
    println!("{} attacked you", npc.get_name());
    calculate_attack(npc, chatacter); 
    println!();

}

