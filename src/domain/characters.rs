pub struct Character { 
        pub name: String,
        pub health: u8,
        pub attack_power:u8,
        pub agiliti:u8
    }

    impl Character { 
       pub fn recieve_damage(&mut self, damage:u8) { 
            let remaining_health = self.health - damage;
            if remaining_health <= 0 { println!("Critic attack, you are KO")}
            else { 
                self.health = remaining_health;
                println!("Health: {}", self.health);
            }
        }

        pub fn attack(&mut self) -> u8 { 
            return self.attack_power;
        }
    }

   pub  struct Goblin { 
        level:u8,
        health: u8,
        attack_power:u8,
        agility:u8
    }

    impl Goblin { 
      pub  fn new(level:u8) -> Goblin { 
            let mut new_goblin  = Goblin { 
                level,
                health : 25 ,
                attack_power: 1, 
                agility :4,
            };

            return new_goblin;
        }
       pub fn recieve_damage(&mut self, damage:u8) { 
            let remaining_health = self.health - damage;
            if remaining_health <= 0 { println!("Critic attack, you are KO")}
            else { 
                self.health = remaining_health;
                println!("Health: {}", self.health);
            }
        }

        pub fn attack(&mut self) -> u8 { 
            return self.attack_power;
        }
}
