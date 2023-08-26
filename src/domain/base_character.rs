pub trait CharacterActions { 
    fn recieve_damage(&mut self, damage:u8) ;
    fn attack(&mut self) -> u8;
    fn get_health(&mut self) -> u8;
    fn get_level(&mut self) -> u8;
    fn get_name(&mut self) -> &str;
    fn print_stats(&mut self) -> ();
    fn restore_partial_health(&mut self) -> ();
}
