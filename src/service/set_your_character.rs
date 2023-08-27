use std::io;

use crate::domain::base_character::CharacterActions;
use crate::domain::characters::Character;
use crate::domain::characters::CharacterBuild;
use crate::service::press_to_continue::press_to_continue;

pub fn set_your_character () -> Character { 
    let set_your_character_title = format!(r#" 

    ___      _                          ___ _                     _           
    / __| ___| |_   _  _ ___ _  _ _ _   / __| |_  __ _ _ _ __ _ __| |_ ___ _ _ 
    \__ \/ -_)  _| | || / _ \ || | '_| | (__| ' \/ _` | '_/ _` / _|  _/ -_) '_|
    |___/\___|\__|  \_, \___/\_,_|_|    \___|_||_\__,_|_| \__,_\__|\__\___|_|  
    |__/                                                       

    "#);
    println!("{}", set_your_character_title);
    println!("Hello stranger, как тебя зовут?: ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name )
        .expect("Failed to read line");
    let mut character: Character;
    loop { 

        print_character_options();
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


        match option { 
            1=> { character =  Character::new(CharacterBuild::Paladin,name); break;},
            2=> { character = Character::new(CharacterBuild::Knight,name); break;},
            3=> { character  = Character::new(CharacterBuild::Vandit,name); break;},
            4 => { print_buid_stats();}
            _=> { println!("Not a valid option")}
        };



    }

    println!("Hello {}", &character.name);
    println!("Your health is {} points", {character.health});
    println!("Your attack power is {} points", {character.attack_power});
    println!("Your agility is {} points", {character.agiliti});

    press_to_continue();

    return character;
}

fn print_buid_stats() ->() { 
    
    let fake_name = String::from(" ");
    
    println!("Printing build stast");
    println!("====================");
    println!();
    println!("Paladin");
    println!("========");
    println!();
    let mut character_to_print  = Character::new(CharacterBuild::Paladin, fake_name.clone() );
    character_to_print.print_stats();
    println!();
    println!("Knight");
    println!("========");
    println!();
    let mut character_to_print  = Character::new(CharacterBuild::Knight, fake_name.clone() );
    character_to_print.print_stats();
    println!();
    println!("Vandit");
    println!("========");
    println!();
    let mut character_to_print  = Character::new(CharacterBuild::Vandit, fake_name.clone() );
    character_to_print.print_stats();
    println!("=========================");

    println!();
    println!();
    press_to_continue();

}

fn print_character_options () -> () { 

    println!("Select your character type:");
    println!("1 - Paladin");
    println!("2 - Kinght");
    println!("3 - Vandit");
    println!("4 - Print builds stats");
}
