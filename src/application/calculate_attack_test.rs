#[cfg(test)]

mod tests {
    

    use crate::{application::calculate_attack::calculate_attack, domain::characters::Character};

    #[test]
    fn defender_sould_recieve_5_points_of_damage(){ 
       let  name_test = String::from("Name"); 
        let mut attacker = Character { 
            name: name_test.clone(),
            health : 50,
            attack_power : 5, 
            agiliti :0, // This way can evade an attack; 
        };

        let mut defender = Character { 
            name: name_test.clone(),
            health : 50,
            attack_power : 6, 
            agiliti :0, // This way can evade an attack; 
        
        };

        let expected_remaining_health:u8 = 45;

        
        calculate_attack(&mut attacker, &mut defender);


        assert_eq!(defender.health, expected_remaining_health);

    }
}
