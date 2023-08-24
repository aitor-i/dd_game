use crate::domain::characters::Character;
use crate::domain::npc::Npc;

pub enum CharEnum <'lifetime>{ 
    Npc(Npc <'lifetime>),
    Character(Character)
}
