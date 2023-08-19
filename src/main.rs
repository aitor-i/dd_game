use std::io;
use rand::Rng;

use crate::domain::characters::Character;

mod domain { 
    pub mod characters;
}

fn main() {
    let game_title = format!(r#" 

______ _                               _   _           ______                                    
|  _  (_)                             | | | |          |  _  \                                   
| | | |_ ___  ___ _____   _____ _ __  | |_| |__   ___  | | | |_   _ _ __   __ _  ___  ___  _ __  
| | | | / __|/ __/ _ \ \ / / _ \ '__| | __| '_ \ / _ \ | | | | | | | '_ \ / _` |/ _ \/ _ \| '_ \ 
| |/ /| \__ \ (_| (_) \ V /  __/ |    | |_| | | |  __/ | |/ /| |_| | | | | (_| |  __/ (_) | | | |
|___/ |_|___/\___\___/ \_/ \___|_|     \__|_| |_|\___| |___/  \__,_|_| |_|\__, |\___|\___/|_| |_|
                                                                           __/ |                 
                                                                          |___/                  

"#);
    println!("{}", game_title);
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


        



    fn set_your_character () -> Character { 
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
}
