use crate::domain::base_character::CharacterActions;
use crate::domain::goblin::Goblin;
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
            _ => println!("{} is not a valid option", {option})
        }
    }   
}

pub    fn calculate_attack(attacker: &mut dyn CharacterActions , defender: &mut dyn CharacterActions) -> () { 
    println!("Attacking...");
    println!("Initial heath: {}", defender.get_health());
    defender.recieve_damage( attacker.attack() );

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
    println!();
    println!();
}

