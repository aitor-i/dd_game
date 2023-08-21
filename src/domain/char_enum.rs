use crate::domain::goblin::Goblin; 
use crate::domain::characters::Character;

pub enum CharEnum { 
    Goblin(Goblin),
    Character(Character)
}
