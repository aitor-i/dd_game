use crate::domain::base_character::CharacterActions;

pub struct Character { 
    pub name: String,
    pub health: u8,
    pub attack_power:u8,
    pub agiliti:u8
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

    fn print_stats(&mut self) -> () {
        println!("Name: {}", self.name);
        println!("Health: {}", self.health);
        println!("Attack power: {}", self.attack_power);
        println!("Agility: {}", self.agiliti);
    }
}

