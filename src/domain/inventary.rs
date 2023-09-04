pub struct Inventary {
    pub items:Vec<InventaryItem>

}

impl Inventary { 
    fn add_to_inventary(mut self, item: InventaryItem)-> () {
        println!("Item added to inventary!" );
        self.items.push(item); 
    }

    fn use_item(self, item_name:InventaryItemsEnum)-> () { 
        for mut item in self.items { 
            if item.name == item_name { 
                if item.times >=  1 { item.times = item.times - 1 }
                println!("item used");

            }
        }
        
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
#[derive(PartialEq)]
 pub enum InventaryItemsEnum { HealingPotion, AgilityPotion, MaxHealthPotion  }
