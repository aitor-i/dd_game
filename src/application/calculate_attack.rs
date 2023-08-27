use crate::{domain::base_character::CharacterActions, service::press_to_continue::press_to_continue};
    
pub fn calculate_attack(attacker: &mut dyn CharacterActions , defender: &mut dyn CharacterActions) -> () { 
    println!("Attacking...");
    println!("Initial heath: {}", defender.get_health());
    defender.recieve_damage( attacker.attack() );

    // The defender counter attacks
    press_to_continue();
    println!("{}, is attacked you!", {defender.get_name()});
    attacker.recieve_damage(defender.attack());

}
