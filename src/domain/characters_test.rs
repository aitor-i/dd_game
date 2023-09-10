#[cfg(test)]

mod tests{ 
    use crate::domain::base_character::CharacterActions;
    use crate::domain::characters::CharacterBuild;
    use crate::domain::characters::Character;

    #[test]
    fn should_create_a_charcter_with_paladin_build_stats() { 
        let name_test = String::from("John"); 
        let paladin_build_stats = Character { 
            name: name_test.clone(),
            health : 54,
            attack_power : 6, 
            agiliti :2, 
        };
        let mut paladin = Character::new(CharacterBuild::Paladin, name_test.clone());

        assert_eq!(paladin.get_health(), paladin_build_stats.health, "Paladin stats sould match" );
        assert_eq!(paladin.get_agility(), paladin_build_stats.agiliti, "Paladin stats sould match" );
    }

    #[test]
    fn characters_should_recieve_damage() { 
        let name_test = String::from("John"); 
        let mut paladin_mock = Character { 
            name: name_test.clone(),
            health : 54,
            attack_power : 6, 
            agiliti :0, // This way cant never evade an attack; 
        };
        let damage:u8 = 5;
        let expected_remaining_health:u8 = 49;

        paladin_mock.recieve_damage(damage);

        assert_eq!(paladin_mock.get_health(), expected_remaining_health, "Character should recieve 5 points of damage");
        
    }

    #[test]
    fn charcter_should_restore_partial_health() { 
        let name_test = String::from("John"); 
        let mut paladin_mock = Character { 
            name: name_test.clone(),
            health : 50,
            attack_power : 6, 
            agiliti :10, // This way cant never evade an attack; 
        };
        let expected_remaining_health:u8 = 55;

        paladin_mock.restore_partial_health();

        assert_eq!(paladin_mock.get_health(), expected_remaining_health, "Character should restore 10% of actual health heath" );
    }


}
