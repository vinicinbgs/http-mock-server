use std::io;

mod account;

fn main() {
    println!("Please, enter your name");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Hmmm... Sorry I did't");

    println!("Please, enter your value");
    let mut value = String::new();
    
    io::stdin()
        .read_line(&mut value)
        .expect("Hmmm... Sorry I did't");

    account::get_balance(&name, &value);
}
