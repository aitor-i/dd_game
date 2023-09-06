pub struct Inventary {
    pub  items:Vec<InventaryItem>

}

impl Inventary { 
    
    pub fn new() -> Self { 
        Self { 
            items: vec![],
        }
    }
    
   pub fn add_to_inventary(&mut self, item: InventaryItem)-> () {
        println!("Item added to inventary!" );
        self.items.push(item); 
    }

   pub fn use_item(&mut self, item_name:InventaryItemsEnum)-> () { 
        for item in &mut self.items { 
            if item.name == item_name { 
                if item.times >=  1 { item.times = item.times - 1 }
                println!("Remaining: {}", item.times);
            }
        }
        let filterd_items : Vec<InventaryItem> = self.items.iter().filter(|&ref item| item.times > 0).cloned().collect();
        println!("Filterd items {:#?}", filterd_items);
        self.items = filterd_items;
        
    }
    
}
#[derive(Debug)]
#[derive(Clone)]
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

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
 pub enum InventaryItemsEnum { HealingPotion, AgilityPotion, MaxHealthPotion  }
