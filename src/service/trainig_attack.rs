use crate::application::npc_actions::npc_actions;
use crate::domain::base_character::CharacterActions;
use crate::domain::goblin::Goblin;
use crate::application::check_stats::check_stats;
use crate::application::calculate_attack::calculate_attack;
use std::io;

pub fn training_fight(character: &mut dyn CharacterActions) -> () { 
    println!("A goblin apear on yor way, it seems to be dangerous");
    println!("What dou you wanna do?");

    let mut goblin = Goblin::new(1, "Small goblin");
    //let mut goblin2 = Goblin::new(1, "Medium Goblin");


    loop { 
        fight_option();

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read option!");

        let option: u8 = match option.trim().parse() { 
            Ok(num)=> num,
            Err(_) => { 
                println!("{} is not a valid option", option);
                continue;
            }
        };

        match option { 
            1 => {calculate_attack(character, &  mut goblin);}
            2 => {check_opponent(&mut goblin)},
            3 => { check_stats(character) }
            _ => println!("{} is not a valid option", {option})
        }
        npc_actions(&mut goblin, character);
    }   
}

pub fn check_opponent(opponent: &mut dyn CharacterActions)-> () { 
    println!("Checking {}", opponent.get_name());
    println!("###############################");
    println!();
    opponent.print_stats();

}

pub  fn fight_option() -> () { 
    println!("1 - Attack");
    println!("2 - Check opponent");
    println!("3 - Check my stats");
    println!();
    println!();
}

