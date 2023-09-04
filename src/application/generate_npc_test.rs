#[cfg(test)]

mod tests{ 
    use crate::application::genrate_npc::generate_npc;
    use crate::application::genrate_npc::NpcType;
    use crate::domain::base_character::CharacterActions;

    #[test]
    fn generate_npc_test()  { 
        println!("Testing generate npc");
        let mut  npc = generate_npc(NpcType::Goblin);
        assert_eq!(npc.get_name(), "Small Goblin");
    }

    #[test]
    fn validate_npc() { 
        println!("Validating!!!!!!")
    }


}
