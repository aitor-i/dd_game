use crate::application::genrate_npc::generate_npc;
use crate::application::npc_actions::npc_actions;
use crate::domain::base_character::CharacterActions;
use crate::application::check_stats::check_stats;
use crate::application::calculate_attack::calculate_attack;
use std::io;
use crate::application::genrate_npc::NpcType;

pub fn training_fight(character: &mut dyn CharacterActions) -> () { 
    println!("A goblin apear on yor way, it seems to be dangerous");
    println!("What dou you wanna do?");


    println!("Select your enemy");
    print_enemy_options();
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read option!");

    let option: u8 = match option.trim().parse() { 
        Ok(num)=> num,
        Err(_) => { 
            println!("{} is not a valid option", option);
            1
        }
    };

    let mut npc= match option { 
        1=> {generate_npc(NpcType::Goblin)},
        2=> {generate_npc(NpcType::Troll)},
        _=> {generate_npc(NpcType::Goblin)},
    };


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
            1 => {calculate_attack(character, &  mut npc);}
            2 => {check_opponent(&mut npc)},
            3 => { check_stats(character) }
            _ => println!("{} is not a valid option", {option})
        }
        npc_actions(&mut npc, character);
        character.restore_partial_health();
        npc.restore_partial_health();
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
fn print_enemy_options() -> () {
    
    println!("1 - Goblin");
    println!("2 - Troll");
}
