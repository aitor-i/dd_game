use crate::domain::base_character::CharacterActions;
    
pub fn calculate_attack(attacker: &mut dyn CharacterActions , defender: &mut dyn CharacterActions) -> () { 
    println!("Attacking...");
    println!("Initial heath: {}", defender.get_health());
    defender.recieve_damage( attacker.attack() );

}
