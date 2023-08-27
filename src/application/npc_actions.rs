use crate::domain::base_character::CharacterActions;

pub fn npc_actions (npc: &mut dyn CharacterActions, chatacter: &mut dyn CharacterActions) -> () { 

    println!();
    println!("{} attacked you", npc.get_name());
    npc_attack(npc, chatacter);
    println!();

}

fn npc_attack(npc: &mut dyn CharacterActions, chatacter: &mut dyn CharacterActions)-> () { 
    chatacter.recieve_damage(npc.attack());
}

