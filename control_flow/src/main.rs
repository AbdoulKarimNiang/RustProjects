use std::io;
//use:rand:Rnd


fn main() {
    println!("Please insert a number to check if it is even");
    let mut user_input: String = String::new();
    
    io::stdin()
    .read_line(&mut user_input)
    .expect("Something wrong while reading");

    let user_input: i32 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("Something went wrong {:?}", e);
            -1
        }

    };

    if user_input % 2 == 0 {
        println!("This is a even number");
    } else {
        println!("This is odd");
    }
}