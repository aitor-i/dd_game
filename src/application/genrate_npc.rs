use crate::domain::npc::Npc;

pub enum NpcType {Goblin, Troll}


pub fn generate_npc(npc_type:NpcType) -> Npc<'static>  { 

    let mut npc: Npc = match npc_type { 
        NpcType::Goblin => {return Npc::new(1, "Small Goblin"); },
        NpcType::Troll => { return Npc::new(4, "Troll"); }
    };
}
