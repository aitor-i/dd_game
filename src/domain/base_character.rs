pub trait CharacterActions { 
    fn recieve_damage(&mut self, damage:u8) ;
    fn attack(&mut self) -> u8;
    fn get_health(&mut self) -> u8;
}