pub struct Inventary {
    pub items:Vec<InventaryItem>

}

impl Inventary { 
    fn add_to_inventary(mut self, item: InventaryItem)-> () {
        println!("Item added to inventary!" );
        self.items.push(item); 
    }

    fn use_item(mut self, item_name:InventaryItemsEnum)-> () { 
        
    }

}


pub struct InventaryItem { 
    name: InventaryItemsEnum,
    description: String,
    power: u8,
    effect_time: u8,
    times: u8
}

impl InventaryItem { 
    pub fn new(name: InventaryItemsEnum, description:String, power:u8, effect_time:u8) -> Self { 
         Self  { 
            name,
            description,
            power,
            effect_time,
            times: 1 as u8
        }
    }
}
 pub enum InventaryItemsEnum { HealingPotion, AgilityPotion, MaxHealthPotion  }
