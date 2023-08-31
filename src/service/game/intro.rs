use crate::service::{messages::print_title, press_to_continue::press_to_continue};

pub fn print_intro_story() -> () { 
    print_title();
    println!("Once upon a time, in a mystical realm known as Eldoria, a great darkness threatened to engulf the land. The people of Eldoria lived in fear, for they knew not what peril lay ahead. In this dire hour, a courageous band of adventurers emerged, their fate entwined by a shared destiny. Each possessed unique skills, from the nimble rogue to the wise mage and the stalwart knight.

");
    press_to_continue();
    println!("Guided by an ancient prophecy, they embarked on a quest to retrieve the lost shards of the Crystal of Lumina, a powerful artifact said to banish darkness and restore light to their world. Their journey would be fraught with peril, and the shadows of Eldoria would test their mettle. But together, they would rise, for the fate of Eldoria rested in their hands.");




}
