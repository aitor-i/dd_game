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
        for mut item in self.items.clone() { 
            if item.name == item_name { 
                if item.times >=  1 { item.times = item.times - 1 }
                println!("item used");
            }
        }
        self.clean_empty_items();
        
    }
    
    fn clean_empty_items(&mut self,) -> () { 
        let index_of_empty_itmes = self.items.iter().enumerate().map(| (index, &ref item )| { if item.times == 0  { return index } else { return 99}  });
        let mut  items_copy = self.items.clone();
        for index in index_of_empty_itmes { 
            if index as u8 != 99 as u8 { 
                items_copy.remove(index);
            }
        }
        self.items = items_copy
    }
}

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
#[derive(PartialEq)]
#[derive(Clone)]
 pub enum InventaryItemsEnum { HealingPotion, AgilityPotion, MaxHealthPotion  }
