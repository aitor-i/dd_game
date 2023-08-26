use crate::domain::base_character::CharacterActions;

pub  struct Npc<'lifetime> { 
    level:u8,
    health: u8,
    attack_power:u8,
    agility:u8,
    name: &'lifetime str
}

impl <'lifetime> Npc <'_>{ 
    pub  fn new(level:u8, name: &'lifetime str) -> Npc {
        let mut new_goblin  = Npc { 
            level,
            health : calculate_stats(10, level, 100) ,
            attack_power: calculate_stats(1, level, 10 ), 
            agility :calculate_stats(1, level, 10),
            name
        };

        return new_goblin;
    }
}


fn calculate_stats(base:u8, level:u8, max:u8) -> u8 { 
    let stat = base * level;
    if stat > max { return max}
    return stat
}

impl CharacterActions for Npc <'_> { 
    
     fn recieve_damage(&mut self, damage:u8) { 
        let remaining_health = self.health - damage;
        if remaining_health <= 0 { println!("Critic attack, you are KO")}
        else { 
            self.health = remaining_health;
            println!("Health: {}", self.health);
        }
    }
    
    fn restore_partial_health(&mut self) -> () {
       let ten_percent = self.health as f32 * 0.1;
        let mut inclemented_health = self.health + ten_percent as u8;
        if inclemented_health > 100 { inclemented_health = 100;};
        self.health = inclemented_health;
        println!("{}'s health restored! Current health: {}",self.name, self.health);
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
