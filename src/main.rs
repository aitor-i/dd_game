use std::io;
mod domain;

use crate::domain::characters::Character;
mod service;
use crate::service::messages::print_title;
use crate::service::set_your_character::set_your_character;

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
            2 => println!("Todo"),
            9 => { 
                println!("Quiting ...");
                continue;
            },
            _ => println!("{} is not a valid option", {option})
        }
        println!();
        println!();
    };

    fn print_options() -> () { 
    println!("1 - Set your character");
    println!("2 - ");
    println!("3 - ");
    println!("9 - Exit  ");
    }

}
