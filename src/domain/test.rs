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


}
