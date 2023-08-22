use std::io;
mod domain;
mod service;

use crate::domain::characters::Character;
use crate::service::messages::print_title;
use crate::service::messages::print_options;
use crate::service::set_your_character::set_your_character;
use crate::service::trainig_attack::training_fight;

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
}

