use rand::Rng;
use std::io;
use std::cmp::Ordering;


fn main() {
    println!("Game: Guessing the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..50);
    let mut tries: u32 = 0;
   
    loop {
        println!("Enter a number: ");
        let mut guess_number: String = String::new();
    
        io::stdin().read_line(&mut guess_number).expect("Failed to read the number!");
        
        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(number) => number,
            Err(_) => continue
        };
    
        tries += 1;
        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Your number is less than secret number!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            },
            Ordering::Greater => println!("Your number is greater than secret number!"),
    
        }
    }

    println!("Did you try: {tries} times!");
}
