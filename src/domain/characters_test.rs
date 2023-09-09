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
    
}
