#[cfg(test)]
mod tests{
use crate::application::can_defend_attack::can_defend_attack;
    use crate::domain::characters::Character;

    #[test]
    fn should_deffend_attack() { 
        
        let name_test = String::from("John"); 
        //let mut paladin_mock = Character::new(CharacterBuild::Paladin, name_test.clone());
        let mut paladin_mock = Character { 
            name: name_test.clone(),
            health : 50,
            attack_power : 6, 
            agiliti :10, // This way can evade an attack; 
        } ;


        let is_defended = can_defend_attack(& mut paladin_mock);

        assert_eq!(is_defended, true);

    }
    #[test]
    fn should_not_deffend_attack() { 
        
        let name_test = String::from("John"); 
        //let mut paladin_mock = Character::new(CharacterBuild::Paladin, name_test.clone());
        let mut paladin_mock = Character { 
            name: name_test.clone(),
            health : 50,
            attack_power : 6, 
            agiliti :0, // This way can evade an attack; 
        } ;


        let is_defended = can_defend_attack(& mut paladin_mock);

        assert_eq!(is_defended, false);

    }
}
