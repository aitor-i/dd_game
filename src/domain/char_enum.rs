use crate::domain::goblin::Goblin; 
use crate::domain::characters::Character;

pub enum CharEnum <'lifetime>{ 
    Goblin(Goblin <'lifetime>),
    Character(Character)
}
