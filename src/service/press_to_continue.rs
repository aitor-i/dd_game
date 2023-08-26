use std::io;

pub fn press_to_continue() -> () { 

    println!("Press any key to continue");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line!");
}
