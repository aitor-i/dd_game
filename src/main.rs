use std::io;
mod domain;
mod service;
mod application;

use crate::domain::inventary::Inventary;
use crate::service::messages::print_title;
use crate::service::messages::print_options;
use crate::service::set_your_character::set_your_character;
use crate::service::trainig_attack::training_fight;
use crate::service::game::intro::print_intro_story;

fn main() {
    print_title();
    let mut character = set_your_character();
    let mut inventay: Inventary;
    
    
    println!("Please select one of this options");
    
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
            1 => {run_history_mode(); },
            2 => { training_fight(&mut character) },
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

fn run_history_mode( )-> () { 
    print_intro_story();
}
