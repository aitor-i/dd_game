use crate::domain::base_character::CharacterActions;

pub  struct Goblin<'lifetime> { 
    level:u8,
    health: u8,
    attack_power:u8,
    agility:u8,
    name: &'lifetime str
}

impl <'lifetime> Goblin <'_>{ 
    pub  fn new(level:u8, name: &'lifetime str) -> Goblin { 
        let mut new_goblin  = Goblin { 
            level,
            health : 25 ,
            attack_power: 1, 
            agility :4,
            name
        };

        return new_goblin;
    }
}

impl CharacterActions for Goblin <'_> { 
    
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
    fn get_level(&mut self) -> u8 { 
        return self.level;
    }
    fn get_name(&mut self) -> &str {
        return self.name;
    }

    fn print_stats(&mut self) -> () {
        println!("Name: {}", self.name);
        println!("Health: {}", self.health);
        println!("Attack power: {}", self.attack_power);
        println!("Agility: {}", self.agility);
    }

}
