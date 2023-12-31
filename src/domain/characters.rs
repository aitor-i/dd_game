use crate::domain::base_character::CharacterActions;

pub struct Character { 
    pub name: String,
    pub health: u8,
    pub attack_power:u8,
    pub agiliti:u8
}

pub enum CharacterBuild {Paladin, Knight, Vandit }

impl Character { 
   pub fn new(character_type: CharacterBuild, name:String) -> Character { 
        return match character_type { 
            CharacterBuild::Paladin => { Character {
                name : String::from(name),
                health : 54,
                attack_power : 6, 
                agiliti :2, 
            }},

            CharacterBuild::Knight=> { Character {
                name : String::from(name),
                health : 47,
                attack_power : 7, 
                agiliti :3, 
            }},
            CharacterBuild::Vandit=> { Character {
                name : String::from(name),
                health : 34,
                attack_power : 3, 
                agiliti :7, 
            }},
        } 

    }
}

impl CharacterActions for Character { 
    fn recieve_damage(&mut self, damage:u8) { 
        let remaining_health = self.health - damage;
        if remaining_health <= 0 { println!("Critic attack, you are KO")}
        else { 
            self.health = remaining_health;
            println!("Health: {}", self.health);
        }
    }

    fn attack(&mut self) -> u8 { 
        return self.attack_power;
    }
    fn get_health(&mut self)-> u8 {
        return self.health;
    }
    fn get_name(&mut self) -> &str { 
        return &self.name;
    }

    fn get_level(&mut self) -> u8 {
        return self.attack_power;
    }

    fn restore_partial_health(&mut self) -> () {
        let ten_percent = self.health as f32 * 0.1;
        let mut inclemented_health = self.health + ten_percent as u8;
        if inclemented_health > 100 { inclemented_health = 100;};
        self.health = inclemented_health;
        println!("Your health has been restored! Current health: {}", self.health);
    }
    fn get_agility(&mut self) -> u8 {
        self.agiliti
    }
    fn print_stats(&mut self) -> () {
        println!("Health: {}", self.health);
        println!("Attack power: {}", self.attack_power);
        println!("Agility: {}", self.agiliti);
    }
}

