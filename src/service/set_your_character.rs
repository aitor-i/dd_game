use std::io;

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

    let mut character = match option { 
        1=> { Character::new(CharacterBuild::Paladin,name)},
        2=> { Character::new(CharacterBuild::Knight,name)},
        3=> { Character::new(CharacterBuild::Vandit,name)},
        _=> { Character::new(CharacterBuild::Paladin,name)}
    };


    println!("Hello {}", &character.name);
    println!("Your health is {} points", {character.health});
    println!("Your attack power is {} points", {character.attack_power});
    println!("Your agility is {} points", {character.agiliti});

    press_to_continue();

    return character;
}


fn print_character_options () -> () { 

    println!("Select your character type:");
    println!("1 - Paladin");
    println!("2 - Kinght");
    println!("3 - Vandit");
}
