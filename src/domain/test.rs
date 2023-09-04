#[cfg(test)]
mod tests {
    
    use crate::domain::base_character::CharacterActions;

    use super::super::npc::Npc;


    #[test]
    fn generate_level_one_npc() { 
        let level: u8 = 1;
        let name = "Goblin_test";
        let mut npc = Npc::new(level, name);
        
        assert_eq!(npc.get_level(), level, "Level should be: {}", level );

    }

    #[test]
    fn npc_should_recieve_damage() { 
        let level: u8 = 1;
        let name = "Goblin_test";
        let mut npc = Npc::new(level, name);
        let damage :u8 = 5;
        let base_health = npc.get_health();

        let remaing_helth = base_health - damage;

        npc.recieve_damage(damage);

        assert_eq!(npc.get_health(), remaing_helth, "Health should be {}", remaing_helth);

    }


}
