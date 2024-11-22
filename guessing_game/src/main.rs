use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    println!("Secret Number");

    let secret_number: u32 = rand::thread_rng().gen_range(0..100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Guess the number!");

        println!("Please input the guess.");

        let mut guess = String::new();

        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
    
        println!("You guessed {}", guess);


        match guess.cmp(&secret_number) { 
            Ordering::Less => println!("The secret number is greater"),
            Ordering::Greater => println!("The secret number is lower"),
            Ordering::Equal => {
                println!("You won!");
                break
            }
        };  
    }
}