use::std::io;

fn main() {

    const _S:i32 = 1_000_000;

    // addition
    let _sum = 5 + 10;
    
    // substraction
    let _sub = 95.3 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3;

    // remainer 
    let _remainer = 43 % 5 ;

    // bolean
    let _t = true;
    let _f:bool = false;

    // char
    let _c: char = 'a'; // single quotes, Unicode Scalar Value

    // tuple
    let tuple:(u32, f64, i16) = (2_500, 4.55, 255);
    let (_x,_y, _z) = tuple;
    
    // println!("The values as x = {}, y = {}, z = {}", x,y,z);

    // You can access values with dot notation

    let _a = tuple.0;

    // Array

    let _array = [1, 2, 3, 4, 5];
    // println!("{:#?}", array); // {:?} list them horizontally while {:#?} list them vertically

    let greater_five: [i8;5] = [6, 7, 8, 9, 10];

    println!("The array is the following:");
    println!("{:#?}", greater_five);
    
    println!("Please insert an index from 0 to 4 to select a value: ");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("The index inserted is not correct");

    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a number or value too big!");
            return; // Exit the function if the parse fails
        }
    };

    let selected_value = greater_five[index];

    println!("You seleted the value {selected_value}"); 
}
