use std::io;
mod domain;
mod service;

use crate::domain::char_enum;
use crate::domain::characters::Character;
use crate::domain::goblin::Goblin;
use crate::service::messages::print_title;
use crate::service::messages::print_options;
use crate::service::set_your_character::set_your_character;
use crate::domain::base_character::CharacterActions;

fn main() {
    print_title();
    println!("Please select one of this options");
    
    let mut character = Character { 
        name : String::new(),
        health : 0,
        attack_power : 0,
        agiliti : 0

    };

    loop {  

        print_options();
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
            1 => {  character = set_your_character()},
            2 => { training_fight() },
            9 => { 
                println!("Quiting ...");
                continue;
            },
            _ => println!("{} is not a valid option", {option})
        }
        println!();
        println!();
    };

    fn training_fight() -> () { 
        println!("A goblin apear on yor way, it seems to be dangerous");
        println!("What dou you wanna do?");
        
        let mut goblin = Goblin::new(1);
        let mut goblin2 = Goblin::new(1);


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
                1 => {calculate_attack(& mut goblin2, &  mut goblin);}
                2 => println!("Todo"),
                _ => println!("{} is not a valid option", {option})
            }
        }   
    }

    fn calculate_attack(attacker: &mut dyn CharacterActions , defender: &mut dyn CharacterActions) -> () { 
        println!("Attacking...");
        println!("Initial heath: {}", defender.get_health());
        defender.recieve_damage( attacker.attack() );
        
    }

    fn fight_option() -> () { 
        println!("1 - Attack");
        println!("2 - Defend");
    }

}
