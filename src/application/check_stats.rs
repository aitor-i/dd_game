use crate::domain::base_character::CharacterActions;

pub fn check_stats(opponent: &mut dyn CharacterActions)-> () { 
    println!("Checking {}", opponent.get_name());
    println!("###############################");
    println!();
    opponent.print_stats();

}

