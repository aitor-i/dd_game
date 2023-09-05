
#[cfg(test)]
mod tests {
    use crate::domain::inventary::{Inventary, InventaryItem, InventaryItemsEnum};

    #[test]
    fn add_item_to_invetary(){  
        let mut inventary: Inventary = Inventary::new();
        let power: u8 = 1;
        let effect_time: u8 = 1;
        let description = String::from("This potion heals!");
        let mut item = InventaryItem::new(InventaryItemsEnum::HealingPotion, description, power, effect_time);

        inventary.add_to_inventary(item);

        assert_eq!(inventary.items.len() , 1);

    }

}
