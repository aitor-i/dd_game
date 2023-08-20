use std::io;
use rand::Rng;

use crate::domain::characters::Character;

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


    let mut character = Character { 
        name : String::from(name),
        health : rand::thread_rng().gen_range(25..100),
        attack_power : rand::thread_rng().gen_range(1..10), 
        agiliti : rand::thread_rng().gen_range(1..10),

    };


    println!("Hello {}", &character.name);
    println!("Your health is {} points", {character.health});
    println!("Your attack power is {} points", {character.attack_power});
    println!("Your agility is {} points", {character.agiliti});

    println!("Press any key to continue");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line!");

    return character;
}
